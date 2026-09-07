#!/usr/bin/env python3
"""Render and install the managed use-subagents-codex worker profile."""

from __future__ import annotations

import argparse
import difflib
import os
import re
import secrets
import stat
import sys
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
    expected = {**tomllib.loads(template), **values}
    if tomllib.loads(rendered) != expected:
        raise ValueError(
            "rendering must change only the top-level worker model and reasoning effort"
        )
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


def open_agents_directory(codex_home: Path, *, create: bool) -> int | None:
    """Pin a real agents directory; never follow a symlink at that boundary."""
    directory = codex_home / "agents"
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
    try:
        return os.open(directory, flags)
    except FileNotFoundError:
        if not create:
            return None
    codex_home.mkdir(mode=0o700, parents=True, exist_ok=True)
    try:
        directory.mkdir(mode=0o700)
    except FileExistsError:
        pass
    return os.open(directory, flags)


def read_target(directory_fd: int | None) -> tuple[str, os.stat_result | None]:
    if directory_fd is None:
        return "", None
    try:
        descriptor = os.open(
            PROFILE_NAME, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
            dir_fd=directory_fd,
        )
    except FileNotFoundError:
        return "", None
    with os.fdopen(descriptor, "r", encoding="utf-8", newline="") as profile:
        metadata = os.fstat(profile.fileno())
        if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1:
            raise ValueError("managed profile must be a regular file with one link")
        return profile.read(), metadata


def write_private_file(directory_fd: int, name: str, content: str) -> None:
    """Create privately before writing; remove incomplete files on failure."""
    remaining = memoryview(content.encode("utf-8"))
    descriptor = os.open(
        name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW,
        0o600, dir_fd=directory_fd,
    )
    try:
        os.fchmod(descriptor, 0o600)
        while remaining:
            written = os.write(descriptor, remaining)
            if written == 0:
                raise OSError("could not finish writing the private profile file")
            remaining = remaining[written:]
        os.fsync(descriptor)
    except BaseException:
        os.unlink(name, dir_fd=directory_fd)
        raise
    finally:
        os.close(descriptor)


def backup_target(target: Path, current: str, directory_fd: int) -> Path:
    timestamp = datetime.now(UTC).strftime("%Y%m%dT%H%M%S%fZ")
    backup = target.with_name(f"{target.name}.bak.{timestamp}")
    write_private_file(directory_fd, backup.name, current)
    return backup


def atomic_write(target: Path, content: str, directory_fd: int) -> None:
    name = f".{target.name}.{secrets.token_hex(16)}"
    write_private_file(directory_fd, name, content)
    try:
        os.replace(name, target.name, src_dir_fd=directory_fd, dst_dir_fd=directory_fd)
    finally:
        try:
            os.unlink(name, dir_fd=directory_fd)
        except FileNotFoundError:
            pass


def repair_permissions(directory_fd: int, expected: os.stat_result) -> None:
    descriptor = os.open(
        PROFILE_NAME, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
        dir_fd=directory_fd,
    )
    try:
        actual = os.fstat(descriptor)
        if (
            not stat.S_ISREG(actual.st_mode)
            or actual.st_nlink != 1
            or (actual.st_dev, actual.st_ino) != (expected.st_dev, expected.st_ino)
        ):
            raise ValueError("managed profile changed during permission reconciliation")
        os.fchmod(descriptor, 0o600)
    finally:
        os.close(descriptor)


def profile_fingerprint(value: os.stat_result | None) -> tuple[int, ...] | None:
    # Reading may update atime; it is not evidence of a concurrent edit.
    if value is None:
        return None
    return (
        value.st_dev, value.st_ino, value.st_mode, value.st_nlink,
        value.st_size, value.st_mtime_ns, value.st_ctime_ns,
    )


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

    if os.name != "posix":
        raise OSError("profile installation requires POSIX file permissions")
    codex_home = effective_codex_home(args.codex_home)
    target = codex_home / "agents" / PROFILE_NAME
    directory_fd = open_agents_directory(codex_home, create=False)
    try:
        current, metadata = read_target(directory_fd)
        if current == desired and metadata is not None:
            current_mode = stat.S_IMODE(metadata.st_mode)
            if current_mode != 0o600:
                print(f"permissions: {target}: {current_mode:04o} -> 0600")
                if args.dry_run:
                    print(f"dry-run: no changes written to {target}")
                else:
                    assert directory_fd is not None
                    repair_permissions(directory_fd, metadata)
                    print(f"permissions updated: {target}")
            else:
                print(f"unchanged: {target}")
            return 0

        show_diff(target, current, desired)
        if args.dry_run:
            print(f"dry-run: no changes written to {target}")
            return 0

        if directory_fd is None:
            directory_fd = open_agents_directory(codex_home, create=True)
        assert directory_fd is not None
        latest, latest_metadata = read_target(directory_fd)
        if latest != current or profile_fingerprint(latest_metadata) != profile_fingerprint(metadata):
            raise ValueError("managed profile changed during reconciliation; rerun the dry run")
        backup = backup_target(target, current, directory_fd) if metadata is not None else None
        atomic_write(target, desired, directory_fd)
        if backup is not None:
            print(f"backup: {backup}")
        print(f"updated: {target}")
        return 0
    finally:
        if directory_fd is not None:
            os.close(directory_fd)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, tomllib.TOMLDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
