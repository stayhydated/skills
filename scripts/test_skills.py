#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "skills-ref @ git+https://github.com/agentskills/agentskills.git@38a2ff82958afee88dadf4831509e6f7e9d8ef4e#subdirectory=skills-ref",
# ]
# ///
"""Exercise skill validation and worker installation without live agent calls."""

from __future__ import annotations

import importlib.util
import os
import stat
import subprocess
import sys
import tempfile
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKER_ROOT = ROOT / "skills" / "use-subagents-codex"
RENDERER = WORKER_ROOT / "scripts" / "configure_worker_profile.py"
PROFILE_NAME = "use-subagents-codex.toml"


@unittest.skipUnless(os.name == "posix", "installer requires POSIX permissions")
class WorkerProfileTests(unittest.TestCase):
    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.home = self.root / "codex"
        self.target = self.home / "agents" / PROFILE_NAME
        self.template = (WORKER_ROOT / "assets" / PROFILE_NAME).read_text(encoding="utf-8")
        self.defaults = tomllib.loads(self.template)
        self.environment = {
            **os.environ,
            "HOME": str(self.root / "user"),
            "CODEX_HOME": str(self.home),
            "PYTHONDONTWRITEBYTECODE": "1",
        }

    def run_renderer(
        self, *arguments: str, explicit_home: bool = True, expected_code: int = 0
    ) -> subprocess.CompletedProcess[str]:
        command = [sys.executable, str(RENDERER)]
        if explicit_home:
            command.extend(["--codex-home", str(self.home)])
        result = subprocess.run(
            [*command, *arguments],
            capture_output=True,
            text=True,
            env=self.environment,
            timeout=10,
            check=False,
        )
        self.assertEqual(result.returncode, expected_code, result.stdout + result.stderr)
        return result

    def read_profile(self) -> dict[str, object]:
        return tomllib.loads(self.target.read_text(encoding="utf-8"))

    def backups(self) -> list[Path]:
        return sorted(self.target.parent.glob(f"{PROFILE_NAME}.bak.*"))

    def test_fresh_dry_run_creates_nothing(self) -> None:
        result = self.run_renderer("--dry-run")
        self.assertIn("dry-run", result.stdout)
        self.assertFalse(self.home.exists())

    def test_defaults_match_template_and_have_private_permissions(self) -> None:
        self.run_renderer()
        self.assertEqual(self.target.read_text(encoding="utf-8"), self.template)
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o600)
        self.assertEqual(self.backups(), [])

    def test_each_representable_effort_is_preserved(self) -> None:
        # Rendering a value does not establish model or runtime support.
        for effort in ("none", "minimal", "low", "medium", "high", "xhigh", "max", "ultra"):
            with self.subTest(effort=effort):
                self.run_renderer("--model", "test-worker", "--reasoning-effort", effort)
                expected = {**self.defaults, "model": "test-worker", "model_reasoning_effort": effort}
                self.assertEqual(self.read_profile(), expected)

    def test_model_only_override_keeps_bundled_effort(self) -> None:
        self.run_renderer("--model", "test-worker")
        self.assertEqual(self.read_profile(), {**self.defaults, "model": "test-worker"})

    def test_effort_only_override_keeps_bundled_model(self) -> None:
        self.run_renderer("--reasoning-effort", "minimal")
        self.assertEqual(self.read_profile(), {**self.defaults, "model_reasoning_effort": "minimal"})

    def test_invalid_model_is_rejected_without_writes(self) -> None:
        for model in ('bad"model', "bad model", "../other"):
            with self.subTest(model=model):
                self.run_renderer("--model", model, expected_code=2)
                self.assertFalse(self.home.exists())

    def test_invalid_effort_is_rejected_without_writes(self) -> None:
        self.run_renderer("--reasoning-effort", "not-an-effort", expected_code=2)
        self.assertFalse(self.home.exists())

    def test_changed_target_is_backed_up_privately(self) -> None:
        self.target.parent.mkdir(parents=True)
        previous = 'name = "previous"\nmodel = "previous-model"\n'
        self.target.write_text(previous, encoding="utf-8")
        self.target.chmod(0o644)
        self.run_renderer()
        backups = self.backups()
        self.assertEqual(len(backups), 1)
        self.assertEqual(backups[0].read_text(encoding="utf-8"), previous)
        self.assertEqual(stat.S_IMODE(backups[0].stat().st_mode), 0o600)
        self.assertEqual(self.target.read_text(encoding="utf-8"), self.template)
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o600)
        self.assertEqual(list(self.target.parent.glob(f".{PROFILE_NAME}.*")), [])

    def test_changed_dry_run_preserves_target_and_creates_no_backup(self) -> None:
        self.target.parent.mkdir(parents=True)
        previous = 'name = "previous"\n'
        self.target.write_text(previous, encoding="utf-8")
        self.target.chmod(0o644)
        self.run_renderer("--dry-run")
        self.assertEqual(self.target.read_text(encoding="utf-8"), previous)
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o644)
        self.assertEqual(self.backups(), [])

    def test_repeated_install_is_idempotent(self) -> None:
        self.run_renderer()
        before = self.target.stat()
        result = self.run_renderer()
        self.assertIn("unchanged:", result.stdout)
        self.assertEqual(self.target.stat().st_mtime_ns, before.st_mtime_ns)
        self.assertEqual(self.target.read_text(encoding="utf-8"), self.template)
        self.assertEqual(self.backups(), [])

    def test_matching_target_permissions_are_repaired_without_rewrite(self) -> None:
        self.run_renderer()
        self.target.chmod(0o644)
        before = self.target.stat().st_mtime_ns
        self.run_renderer()
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o600)
        self.assertEqual(self.target.stat().st_mtime_ns, before)
        self.assertEqual(self.target.read_text(encoding="utf-8"), self.template)
        self.assertEqual(self.backups(), [])

    def test_permission_dry_run_is_read_only(self) -> None:
        self.run_renderer()
        self.target.chmod(0o644)
        result = self.run_renderer("--dry-run")
        self.assertIn("0644 -> 0600", result.stdout)
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o644)
        self.assertEqual(self.target.read_text(encoding="utf-8"), self.template)
        self.assertEqual(self.backups(), [])

    def test_parent_configuration_is_untouched(self) -> None:
        self.home.mkdir()
        config = self.home / "config.toml"
        original = b'model = "parent-model"\nmodel_reasoning_effort = "low"\n'
        config.write_bytes(original)
        self.run_renderer("--model", "test-worker", "--reasoning-effort", "minimal")
        self.assertEqual(config.read_bytes(), original)

    def test_environment_home_is_used(self) -> None:
        self.run_renderer(explicit_home=False)
        self.assertEqual(self.target.read_text(encoding="utf-8"), self.template)

    def test_explicit_home_wins_over_environment(self) -> None:
        other = self.root / "other-codex"
        self.environment["CODEX_HOME"] = str(other)
        self.run_renderer()
        self.assertTrue(self.target.is_file())
        self.assertFalse(other.exists())

    def test_unset_codex_home_uses_user_home(self) -> None:
        del self.environment["CODEX_HOME"]
        self.run_renderer(explicit_home=False)
        target = Path(self.environment["HOME"]) / ".codex" / "agents" / PROFILE_NAME
        self.assertEqual(target.read_text(encoding="utf-8"), self.template)
        self.assertFalse(self.home.exists())


class SkillValidationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        # Import only for these tests so renderer-only checks need no dependencies.
        spec = importlib.util.spec_from_file_location("check_skills", ROOT / "scripts" / "check_skills.py")
        if spec is None or spec.loader is None:
            raise RuntimeError("cannot load the repository skill validator")
        cls.validator = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.validator)

    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.skill = Path(temporary.name) / "example-skill"
        self.skill.mkdir()

    def write_skill(self, extra: str = "") -> None:
        content = (
            "---\nname: example-skill\n"
            "description: Exercise the validator with a small skill fixture.\n"
            f"{extra}---\n\n# Example\n\nFollow the requested scope.\n"
        )
        (self.skill / "SKILL.md").write_text(content, encoding="utf-8")

    def test_compatibility_is_optional(self) -> None:
        self.write_skill()
        self.assertEqual(self.validator.validate_skill_shape(self.skill), [])
        self.assertEqual(self.validator.validate_agent_skill(self.skill), [])

    def test_compatibility_accepts_valid_strings_and_boundary_length(self) -> None:
        for value in ("Requires Python 3.11+ and POSIX permissions.", "x", "x" * 500):
            with self.subTest(length=len(value)):
                self.write_skill(f'compatibility: "{value}"\n')
                self.assertEqual(self.validator.validate_skill_shape(self.skill), [])
                self.assertEqual(self.validator.validate_agent_skill(self.skill), [])

    def test_compatibility_rejects_empty_or_blank_strings(self) -> None:
        for value in ('""', '"   "'):
            with self.subTest(value=value):
                self.write_skill(f"compatibility: {value}\n")
                errors = self.validator.validate_skill_shape(self.skill)
                self.assertTrue(any("compatibility" in error for error in errors), errors)

    def test_compatibility_rejects_overlong_strings(self) -> None:
        self.write_skill(f'compatibility: "{"x" * 501}"\n')
        errors = self.validator.validate_skill_shape(self.skill)
        self.assertTrue(any("compatibility" in error for error in errors), errors)

    def test_compatibility_rejects_non_string_structures(self) -> None:
        for value in ("\n  - python", "\n  runtime: python"):
            with self.subTest(value=value):
                self.write_skill(f"compatibility: {value}\n")
                errors = self.validator.validate_skill_shape(self.skill)
                self.assertTrue(any("compatibility" in error for error in errors), errors)

    def test_unknown_frontmatter_is_still_rejected(self) -> None:
        self.write_skill("not-a-spec-field: value\n")
        errors = self.validator.validate_skill_shape(self.skill)
        self.assertTrue(any("unsupported frontmatter" in error for error in errors), errors)

    def test_resource_validation_is_preserved(self) -> None:
        self.write_skill()
        resources = self.skill / "references"
        resources.mkdir()
        (resources / "guide.md").write_text("# Guide\n", encoding="utf-8")
        errors = self.validator.validate_skill_shape(self.skill)
        self.assertTrue(any("must be referenced directly" in error for error in errors), errors)
        skill_path = self.skill / "SKILL.md"
        with skill_path.open("a", encoding="utf-8") as skill_file:
            skill_file.write("\nRead [the guide](references/guide.md).\n")
        self.assertEqual(self.validator.validate_skill_shape(self.skill), [])


if __name__ == "__main__":
    unittest.main()
