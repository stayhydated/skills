#!/usr/bin/env -S uv run
"""Validate Agent Skills contracts and repository OpenAI metadata."""

from __future__ import annotations

import sys
from pathlib import Path
from typing import Annotated, Any, Literal

import pydantic
from pydantic import ConfigDict, Field, StringConstraints
from ruamel.yaml import YAML
from ruamel.yaml.error import YAMLError
from skills_ref import validate

REPOSITORY_ROOT = Path(__file__).resolve().parent.parent
SKILLS_ROOT = REPOSITORY_ROOT / "skills"

STRICT_MODEL = ConfigDict(extra="forbid", strict=True)
NonEmptyString = Annotated[str, StringConstraints(strip_whitespace=True, min_length=1)]
ShortDescription = Annotated[
    str, StringConstraints(strip_whitespace=True, min_length=25, max_length=64)
]
BrandColor = Annotated[str, StringConstraints(pattern=r"^#[0-9A-Fa-f]{6}$")]

OPENAI_YAML = YAML(typ="safe")
OPENAI_YAML.allow_duplicate_keys = False


class Policy(pydantic.BaseModel):
    model_config = STRICT_MODEL

    allow_implicit_invocation: bool | None = None
    products: Annotated[list[Literal["CHAT", "CODEX"]], Field(min_length=1)] | None = None


class DependencyTool(pydantic.BaseModel):
    model_config = STRICT_MODEL

    type: Literal["mcp"]
    value: NonEmptyString
    description: NonEmptyString | None = None
    transport: NonEmptyString | None = None
    url: NonEmptyString | None = None


class Dependencies(pydantic.BaseModel):
    model_config = STRICT_MODEL

    tools: list[DependencyTool] = Field(default_factory=list)


class Interface(pydantic.BaseModel):
    model_config = STRICT_MODEL

    display_name: NonEmptyString
    short_description: ShortDescription
    default_prompt: NonEmptyString
    brand_color: BrandColor | None = None
    icon_large: NonEmptyString | None = None
    icon_small: NonEmptyString | None = None


class OpenAIMetadata(pydantic.BaseModel):
    model_config = STRICT_MODEL

    interface: Interface
    policy: Policy = Field(default_factory=Policy)
    dependencies: Dependencies = Field(default_factory=Dependencies)


def pydantic_errors(path: Path, error: pydantic.ValidationError) -> list[str]:
    messages: list[str] = []
    for item in error.errors():
        location = ".".join(str(part) for part in item["loc"])
        detail = f"{location}: {item['msg']}" if location else item["msg"]
        messages.append(f"{path}: {detail}")
    return messages


def load_openai_mapping(
    path: Path, content: str
) -> tuple[dict[str, Any] | None, list[str]]:
    try:
        document = OPENAI_YAML.load(content)
    except YAMLError as error:
        return None, [f"{path}: invalid YAML: {error}"]

    if not isinstance(document, dict):
        return None, [f"{path}: YAML document must be a mapping"]

    return document, []


def validate_openai_metadata(skill_dir: Path) -> list[str]:
    metadata_path = skill_dir / "agents" / "openai.yaml"
    if not metadata_path.is_file():
        return [f"{metadata_path}: required repository UI metadata is missing"]

    content = metadata_path.read_text(encoding="utf-8")
    document, errors = load_openai_mapping(metadata_path, content)
    if document is None:
        return errors

    try:
        OpenAIMetadata.model_validate(document)
    except pydantic.ValidationError as error:
        errors.extend(pydantic_errors(metadata_path, error))

    interface = document.get("interface")
    if isinstance(interface, dict):
        skill_invocation = f"${skill_dir.name}"
        default_prompt = interface.get("default_prompt")
        if isinstance(default_prompt, str) and skill_invocation not in default_prompt:
            errors.append(
                f"{metadata_path}: default_prompt must mention {skill_invocation!r}"
            )

        for icon_field in ("icon_small", "icon_large"):
            icon_path = interface.get(icon_field)
            if isinstance(icon_path, str) and not (skill_dir / icon_path).is_file():
                errors.append(
                    f"{metadata_path}: {icon_field} path {icon_path!r} does not exist"
                )

    return errors


def main() -> int:
    skill_dirs = sorted(
        path for path in SKILLS_ROOT.iterdir() if path.is_dir() and not path.name.startswith(".")
    )
    if not skill_dirs:
        print(f"No skill directories found under {SKILLS_ROOT}", file=sys.stderr)
        return 1

    all_errors: list[str] = []
    for skill_dir in skill_dirs:
        errors = [
            *(f"{skill_dir}: {error}" for error in validate(skill_dir)),
            *validate_openai_metadata(skill_dir),
        ]
        if errors:
            all_errors.extend(errors)
            print(f"Invalid skill: {skill_dir.relative_to(REPOSITORY_ROOT)}")
        else:
            print(f"Valid skill: {skill_dir.relative_to(REPOSITORY_ROOT)}")

    if all_errors:
        print("\nSkill validation failed:", file=sys.stderr)
        for error in all_errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print(f"Validated {len(skill_dirs)} skills.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
