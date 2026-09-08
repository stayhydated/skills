#!/usr/bin/env -S uv run
"""Validate every repository skill against the Agent Skills contract."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Any

import yaml

REPOSITORY_ROOT = Path(__file__).resolve().parent.parent
SKILLS_ROOT = REPOSITORY_ROOT / "skills"
MAX_SKILL_LINES = 500
MAX_COMPATIBILITY_LENGTH = 500
ALLOWED_SKILL_FIELDS = {
    "allowed-tools",
    "compatibility",
    "description",
    "license",
    "metadata",
    "name",
}
SUPPORTED_RESOURCE_DIRECTORIES = (
    "assets",
    "checklists",
    "patterns",
    "references",
    "scripts",
    "templates",
)
RESOURCE_PATH_PATTERN = re.compile(
    r"(?:assets|checklists|patterns|references|scripts|templates)"
    r"/[A-Za-z0-9_.\-/]+"
)
EXTERNAL_URL_START_PATTERN = re.compile(r"\b[A-Za-z][A-Za-z0-9+.-]*://|(?<![\w/])//")
ALLOWED_INTERFACE_FIELDS = {
    "brand_color",
    "default_prompt",
    "display_name",
    "icon_large",
    "icon_small",
    "short_description",
}
REQUIRED_INTERFACE_FIELDS = {
    "default_prompt",
    "display_name",
    "short_description",
}
ALLOWED_OPENAI_FIELDS = {"dependencies", "interface", "policy"}


def validate_agent_skill(skill_dir: Path) -> list[str]:
    from skills_ref import validate

    return validate(skill_dir)


def load_yaml_mapping(path: Path, content: str) -> tuple[dict[str, Any] | None, list[str]]:
    import strictyaml

    try:
        value = strictyaml.load(content).data
    except strictyaml.YAMLError as error:
        return None, [f"{path}: invalid YAML: {error}"]

    if not isinstance(value, dict):
        return None, [f"{path}: YAML document must be a mapping"]

    return value, []


def split_skill(path: Path) -> tuple[dict[str, Any] | None, str, list[str]]:
    content = path.read_text(encoding="utf-8")
    lines = content.splitlines()
    if not lines or lines[0] != "---":
        return None, "", [f"{path}: SKILL.md must start with YAML frontmatter"]

    try:
        frontmatter_end = lines.index("---", 1)
    except ValueError:
        return None, "", [f"{path}: YAML frontmatter is missing its closing delimiter"]

    frontmatter, errors = load_yaml_mapping(
        path,
        "\n".join(lines[1:frontmatter_end]),
    )
    body = "\n".join(lines[frontmatter_end + 1 :]).strip()
    return frontmatter, body, errors


def without_external_urls(content: str) -> str:
    """Exclude URL text from both bundled-resource presence and existence checks."""
    parts: list[str] = []
    cursor = 0
    for match in EXTERNAL_URL_START_PATTERN.finditer(content):
        if match.start() < cursor:
            continue
        end = match.end()
        preceding = content[match.start() - 1] if match.start() else ""
        quote = preceding if preceding in ("'", '"') else None
        closers: list[str] = []
        while end < len(content):
            char = content[end]
            if char.isspace() or char in "<>`" or char == quote:
                break
            if char in "([":
                closers.append(")" if char == "(" else "]")
            elif char in ")]":
                if not closers or closers[-1] != char:
                    break
                closers.pop()
            end += 1
        # Leave Markdown delimiters intact so an adjacent local link is checked.
        parts.extend((content[cursor:match.start()], " "))
        cursor = end
    parts.append(content[cursor:])
    return "".join(parts)


def validate_skill_shape(skill_dir: Path) -> list[str]:
    skill_path = skill_dir / "SKILL.md"
    frontmatter, body, errors = split_skill(skill_path)
    if frontmatter is None:
        return errors

    if not body:
        errors.append(f"{skill_path}: Markdown instruction body must not be empty")

    unexpected_fields = set(frontmatter) - ALLOWED_SKILL_FIELDS
    if unexpected_fields:
        errors.append(
            f"{skill_path}: unsupported frontmatter fields: "
            f"{', '.join(sorted(unexpected_fields))}"
        )

    line_count = len(skill_path.read_text(encoding="utf-8").splitlines())
    if line_count > MAX_SKILL_LINES:
        errors.append(
            f"{skill_path}: keep SKILL.md under {MAX_SKILL_LINES} lines "
            f"for progressive disclosure (found {line_count})"
        )

    for field in ("license", "allowed-tools"):
        if field in frontmatter and not isinstance(frontmatter[field], str):
            errors.append(f"{skill_path}: frontmatter field {field!r} must be a string")

    if "compatibility" in frontmatter:
        compatibility = frontmatter["compatibility"]
        if not isinstance(compatibility, str) or not compatibility.strip():
            errors.append(
                f"{skill_path}: frontmatter field 'compatibility' must be a non-empty string"
            )
        elif len(compatibility) > MAX_COMPATIBILITY_LENGTH:
            errors.append(
                f"{skill_path}: compatibility must contain 1-{MAX_COMPATIBILITY_LENGTH} "
                f"characters (found {len(compatibility)})"
            )

    metadata = frontmatter.get("metadata")
    if metadata is not None:
        if not isinstance(metadata, dict):
            errors.append(f"{skill_path}: frontmatter field 'metadata' must be a mapping")
        elif not all(
            isinstance(key, str) and isinstance(value, str)
            for key, value in metadata.items()
        ):
            errors.append(
                f"{skill_path}: frontmatter metadata keys and values must be strings"
            )

    skill_content = without_external_urls(skill_path.read_text(encoding="utf-8"))
    for resource_directory in SUPPORTED_RESOURCE_DIRECTORIES:
        resource_root = skill_dir / resource_directory
        if not resource_root.is_dir():
            continue
        for resource_path in sorted(path for path in resource_root.rglob("*") if path.is_file()):
            relative_path = resource_path.relative_to(skill_dir).as_posix()
            if relative_path not in skill_content:
                errors.append(
                    f"{skill_path}: bundled resource {relative_path!r} must be "
                    "referenced directly from SKILL.md"
                )

    for resource_reference in sorted(set(RESOURCE_PATH_PATTERN.findall(skill_content))):
        if not (skill_dir / resource_reference).is_file():
            errors.append(
                f"{skill_path}: referenced resource {resource_reference!r} does not exist"
            )

    return errors


class OpenAIMetadataLoader(yaml.SafeLoader):
    """Preserve YAML scalar types and reject ambiguous duplicate/non-string keys."""

    def construct_mapping(self, node: yaml.MappingNode, deep: bool = False) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key_node, value_node in node.value:
            key = self.construct_object(key_node, deep=deep)
            if not isinstance(key, str) or key in result:
                raise yaml.constructor.ConstructorError(
                    "while reading metadata", node.start_mark,
                    "mapping keys must be unique strings", key_node.start_mark,
                )
            result[key] = self.construct_object(value_node, deep=deep)
        return result


# Match the runtime's canonical boolean spelling, not YAML 1.1's yes/on aliases.
OpenAIMetadataLoader.yaml_implicit_resolvers = {
    key: [(tag, pattern) for tag, pattern in resolvers if tag != "tag:yaml.org,2002:bool"]
    for key, resolvers in yaml.SafeLoader.yaml_implicit_resolvers.items()
}
OpenAIMetadataLoader.add_implicit_resolver(
    "tag:yaml.org,2002:bool", re.compile(r"^(?:true|false)$"), ["t", "f"]
)


def load_openai_mapping(path: Path, content: str) -> tuple[dict[str, Any] | None, list[str]]:
    try:
        document = yaml.load(content, Loader=OpenAIMetadataLoader)
    except yaml.YAMLError as error:
        return None, [f"{path}: invalid YAML: {error}"]
    if not isinstance(document, dict):
        return None, [f"{path}: YAML document must be a mapping"]
    return document, []


def validate_openai_nested_metadata(path: Path, document: dict[str, Any]) -> list[str]:
    # Supported fields: OpenAI skill-creator/references/openai_yaml.md and
    # https://developers.openai.com/plugins/deploy/submission-errors
    errors: list[str] = []
    if "policy" in document:
        policy = document["policy"]
        if not isinstance(policy, dict):
            errors.append(f"{path}: 'policy' must be a mapping")
        else:
            unknown = set(policy) - {"allow_implicit_invocation", "products"}
            if unknown:
                errors.append(f"{path}: unsupported policy fields: {', '.join(sorted(unknown))}")
            if "allow_implicit_invocation" in policy and not isinstance(
                policy["allow_implicit_invocation"], bool
            ):
                errors.append(f"{path}: policy.allow_implicit_invocation must be a boolean")
            if "products" in policy:
                products = policy["products"]
                if not isinstance(products, list) or not products or not all(
                    isinstance(product, str) and product in {"CHAT", "CODEX"}
                    for product in products
                ):
                    errors.append(f"{path}: policy.products must list CHAT, CODEX, or both")

    if "dependencies" not in document:
        return errors
    dependencies = document["dependencies"]
    if not isinstance(dependencies, dict):
        errors.append(f"{path}: 'dependencies' must be a mapping")
        return errors
    unknown = set(dependencies) - {"tools"}
    if unknown:
        errors.append(f"{path}: unsupported dependencies fields: {', '.join(sorted(unknown))}")
    if "tools" not in dependencies:
        return errors
    tools = dependencies["tools"]
    if not isinstance(tools, list):
        errors.append(f"{path}: dependencies.tools must be a list")
        return errors
    allowed = {"type", "value", "description", "transport", "url"}
    for index, tool in enumerate(tools):
        label = f"{path}: dependencies.tools[{index}]"
        if not isinstance(tool, dict):
            errors.append(f"{label} must be a mapping")
            continue
        unknown = set(tool) - allowed
        if unknown:
            errors.append(f"{label}: unsupported fields: {', '.join(sorted(unknown))}")
        for field in ("type", "value"):
            if field not in tool:
                errors.append(f"{label}: missing required field {field!r}")
        for field in allowed & set(tool):
            if not isinstance(tool[field], str) or not tool[field].strip():
                errors.append(f"{label}: {field!r} must be a non-empty string")
        if "type" in tool and tool["type"] != "mcp":
            errors.append(f"{label}: only dependency type 'mcp' is supported")
    return errors


def validate_openai_metadata(skill_dir: Path) -> list[str]:
    metadata_path = skill_dir / "agents" / "openai.yaml"
    if not metadata_path.is_file():
        return [f"{metadata_path}: required repository UI metadata is missing"]

    content = metadata_path.read_text(encoding="utf-8")
    document, errors = load_openai_mapping(metadata_path, content)
    if document is None:
        return errors

    errors.extend(validate_openai_nested_metadata(metadata_path, document))

    unexpected_fields = set(document) - ALLOWED_OPENAI_FIELDS
    if unexpected_fields:
        errors.append(
            f"{metadata_path}: unsupported top-level fields: "
            f"{', '.join(sorted(unexpected_fields))}"
        )

    interface = document.get("interface")
    if not isinstance(interface, dict):
        errors.append(f"{metadata_path}: 'interface' must be a mapping")
        return errors

    missing_fields = REQUIRED_INTERFACE_FIELDS - set(interface)
    if missing_fields:
        errors.append(
            f"{metadata_path}: missing interface fields: "
            f"{', '.join(sorted(missing_fields))}"
        )

    unexpected_interface_fields = set(interface) - ALLOWED_INTERFACE_FIELDS
    if unexpected_interface_fields:
        errors.append(
            f"{metadata_path}: unsupported interface fields: "
            f"{', '.join(sorted(unexpected_interface_fields))}"
        )

    for field, value in interface.items():
        if field in ALLOWED_INTERFACE_FIELDS and (
            not isinstance(value, str) or not value.strip()
        ):
            errors.append(
                f"{metadata_path}: interface field {field!r} must be a non-empty string"
            )

    short_description = interface.get("short_description")
    if isinstance(short_description, str) and not 25 <= len(short_description) <= 64:
        errors.append(
            f"{metadata_path}: short_description must contain 25-64 characters "
            f"(found {len(short_description)})"
        )

    default_prompt = interface.get("default_prompt")
    skill_invocation = f"${skill_dir.name}"
    if isinstance(default_prompt, str) and skill_invocation not in default_prompt:
        errors.append(
            f"{metadata_path}: default_prompt must mention {skill_invocation!r}"
        )

    brand_color = interface.get("brand_color")
    if isinstance(brand_color, str) and re.fullmatch(r"#[0-9A-Fa-f]{6}", brand_color) is None:
        errors.append(
            f"{metadata_path}: brand_color must be a six-digit hexadecimal color"
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
            *(f"{skill_dir}: {error}" for error in validate_agent_skill(skill_dir)),
            *validate_skill_shape(skill_dir),
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
