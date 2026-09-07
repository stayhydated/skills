#!/usr/bin/env python3
"""Render and install the managed use-subagents-codex worker profile."""

from __future__ import annotations

import argparse
import difflib
import os
import re
import shutil
import stat
import sys
import tempfile
import tomllib
from datetime import UTC, datetime
from pathlib import Path


PROFILE_NAME = "use-subagents-codex.toml"
SKILL_ROOT = Path(__file__).resolve().parent.parent
TEMPLATE_PATH = SKILL_ROOT / "assets" / PROFILE_NAME
MODEL_PATTERN = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*$")
# Representable values, not a promise of support by any model or runtime.
# The skill verifies the resolved pair against the active spawn surface first.
EFFORTS = ("none", "minimal", "low", "medium", "high", "xhigh", "max", "ultra")
ASSIGNMENTS = {
    "model": re.compile(r'^model\s*=\s*"[^"\n]*"\s*$', re.MULTILINE),
    "model_reasoning_effort": re.compile(
        r'^model_reasoning_effort\s*=\s*"[^"\n]*"\s*$', re.MULTILINE
    ),
}


def model_identifier(value: str) -> str:
    if MODEL_PATTERN.fullmatch(value) is None:
        raise argparse.ArgumentTypeError(
            "model must contain only letters, digits, periods, underscores, and hyphens"
        )
    return value


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Render the bundled custom-agent template with an exact worker model "
            "and reasoning effort, then install it under the effective Codex home."
        )
    )
    parser.add_argument(
        "--model",
        type=model_identifier,
        help="exact worker model identifier; defaults to the bundled value",
    )
    parser.add_argument(
        "--reasoning-effort",
        choices=EFFORTS,
        help=(
            "worker reasoning effort; defaults to the bundled value; "
            "model and runtime support must be verified separately"
        ),
    )
    parser.add_argument(
        "--codex-home",
        type=Path,
        help="target Codex home; defaults to CODEX_HOME or ~/.codex",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="print intended content and permission changes without writing",
    )
    return parser.parse_args()


def load_template() -> tuple[str, dict[str, object]]:
    text = TEMPLATE_PATH.read_text(encoding="utf-8")
    document = tomllib.loads(text)
    for key in ("model", "model_reasoning_effort"):
        if not isinstance(document.get(key), str):
            raise ValueError(f"{TEMPLATE_PATH}: {key!r} must be a TOML string")
        if len(ASSIGNMENTS[key].findall(text)) != 1:
            raise ValueError(
                f"{TEMPLATE_PATH}: expected exactly one top-level {key!r} assignment"
            )
    return text, document


def render(template: str, model: str, effort: str) -> str:
    values = {"model": model, "model_reasoning_effort": effort}
    rendered = template
    for key, value in values.items():
        rendered = ASSIGNMENTS[key].sub(f'{key} = "{value}"', rendered, count=1)
    tomllib.loads(rendered)
    return rendered


def effective_codex_home(explicit_home: Path | None) -> Path:
    if explicit_home is not None:
        return explicit_home.expanduser().resolve()
    configured_home = os.environ.get("CODEX_HOME")
    if configured_home:
        return Path(configured_home).expanduser().resolve()
    return (Path.home() / ".codex").resolve()


def show_diff(target: Path, current: str, desired: str) -> None:
    before = str(target) if target.exists() else "/dev/null"
    diff = difflib.unified_diff(
        current.splitlines(keepends=True),
        desired.splitlines(keepends=True),
        fromfile=before,
        tofile=str(target),
    )
    sys.stdout.writelines(diff)


def backup_target(target: Path) -> Path:
    timestamp = datetime.now(UTC).strftime("%Y%m%dT%H%M%S%fZ")
    backup = target.with_name(f"{target.name}.bak.{timestamp}")
    shutil.copy2(target, backup)
    backup.chmod(0o600)
    return backup


def atomic_write(target: Path, content: str) -> None:
    temporary_path: Path | None = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="w",
            encoding="utf-8",
            dir=target.parent,
            prefix=f".{target.name}.",
            delete=False,
        ) as temporary:
            temporary_path = Path(temporary.name)
            os.fchmod(temporary.fileno(), 0o600)
            temporary.write(content)
            temporary.flush()
            os.fsync(temporary.fileno())
        os.replace(temporary_path, target)
        target.chmod(0o600)
    finally:
        if temporary_path is not None and temporary_path.exists():
            temporary_path.unlink()


def main() -> int:
    args = parse_args()
    template, defaults = load_template()
    model = args.model or str(defaults["model"])
    effort = args.reasoning_effort or str(defaults["model_reasoning_effort"])
    model_identifier(model)
    if effort not in EFFORTS:
        raise ValueError(
            f"{TEMPLATE_PATH}: unsupported bundled reasoning effort {effort!r}"
        )
    desired = render(template, model, effort)

    codex_home = effective_codex_home(args.codex_home)
    target = codex_home / "agents" / PROFILE_NAME
    current = target.read_text(encoding="utf-8") if target.is_file() else ""

    if current == desired:
        current_mode = stat.S_IMODE(target.stat().st_mode)
        if current_mode != 0o600:
            print(f"permissions: {target}: {current_mode:04o} -> 0600")
            if args.dry_run:
                print(f"dry-run: no changes written to {target}")
            else:
                target.chmod(0o600)
                print(f"permissions updated: {target}")
        else:
            print(f"unchanged: {target}")
        return 0

    show_diff(target, current, desired)
    if args.dry_run:
        print(f"dry-run: no changes written to {target}")
        return 0

    target.parent.mkdir(parents=True, exist_ok=True)
    backup = backup_target(target) if target.exists() else None
    atomic_write(target, desired)
    if backup is not None:
        print(f"backup: {backup}")
    print(f"updated: {target}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, tomllib.TOMLDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
