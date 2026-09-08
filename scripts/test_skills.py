#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "PyYAML==6.0.3",
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
from unittest import mock

# Imported bundled helpers must not create resources inside the skill bundles.
sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parent.parent
WORKER_ROOT = ROOT / "skills" / "use-subagents-codex"
RENDERER = WORKER_ROOT / "scripts" / "configure_worker_profile.py"
PROFILE_NAME = "use-subagents-codex.toml"


@unittest.skipUnless(os.name == "posix", "installer requires POSIX permissions")
class WorkerProfileTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        spec = importlib.util.spec_from_file_location("worker_renderer", RENDERER)
        if spec is None or spec.loader is None:
            raise RuntimeError("cannot load the worker renderer")
        cls.renderer = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.renderer)

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

    def test_profile_symlinks_are_rejected_without_touching_referents(self) -> None:
        self.target.parent.mkdir(parents=True)
        outside = self.root / "outside.toml"
        for content in (self.template, 'name = "different"\n', None):
            for arguments in ((), ("--dry-run",)):
                with self.subTest(content=content is not None, arguments=arguments):
                    if content is not None:
                        outside.write_text(content, encoding="utf-8")
                        outside.chmod(0o644)
                    self.target.symlink_to(outside)
                    self.run_renderer(*arguments, expected_code=1)
                    self.assertTrue(self.target.is_symlink())
                    if content is None:
                        self.assertFalse(outside.exists())
                    else:
                        self.assertEqual(outside.read_text(encoding="utf-8"), content)
                        self.assertEqual(stat.S_IMODE(outside.stat().st_mode), 0o644)
                        outside.unlink()
                    self.assertEqual(self.backups(), [])
                    self.target.unlink()

    def test_symlinked_agents_directory_is_rejected(self) -> None:
        self.home.mkdir()
        outside = self.root / "outside-agents"
        agents = self.home / "agents"
        for exists in (True, False):
            for arguments in ((), ("--dry-run",)):
                with self.subTest(exists=exists, arguments=arguments):
                    if exists:
                        outside.mkdir()
                    agents.symlink_to(outside, target_is_directory=True)
                    self.run_renderer(*arguments, expected_code=1)
                    self.assertTrue(agents.is_symlink())
                    if exists:
                        self.assertEqual(list(outside.iterdir()), [])
                        outside.rmdir()
                    else:
                        self.assertFalse(outside.exists())
                    agents.unlink()

    def test_hardlinked_target_is_rejected(self) -> None:
        self.target.parent.mkdir(parents=True)
        outside = self.root / "outside.toml"
        outside.write_text(self.template, encoding="utf-8")
        outside.chmod(0o644)
        os.link(outside, self.target)
        self.run_renderer(expected_code=1)
        self.assertEqual(stat.S_IMODE(outside.stat().st_mode), 0o644)
        self.assertEqual(outside.read_text(encoding="utf-8"), self.template)
        self.assertEqual(self.backups(), [])

    def test_fifo_target_is_rejected_without_blocking(self) -> None:
        self.target.parent.mkdir(parents=True)
        os.mkfifo(self.target)
        self.run_renderer(expected_code=1)
        self.assertTrue(stat.S_ISFIFO(self.target.lstat().st_mode))
        self.assertEqual(self.backups(), [])

    def open_directory(self) -> int:
        descriptor = self.renderer.open_agents_directory(self.home, create=True)
        self.assertIsNotNone(descriptor)
        self.addCleanup(os.close, descriptor)
        return descriptor

    def test_backup_is_private_before_first_byte_is_written(self) -> None:
        descriptor = self.open_directory()
        current = 'name = "private"\n'
        self.target.write_text(current, encoding="utf-8")
        self.target.chmod(0o600)
        observed: list[int] = []
        write = os.write

        def inspect_write(fd: int, data: bytes) -> int:
            observed.append(stat.S_IMODE(os.fstat(fd).st_mode))
            return write(fd, data)

        umask = os.umask(0o022)
        try:
            with mock.patch.object(self.renderer.os, "write", side_effect=inspect_write):
                backup = self.renderer.backup_target(self.target, current, descriptor)
        finally:
            os.umask(umask)
        self.assertTrue(observed)
        self.assertEqual(set(observed), {0o600})
        self.assertEqual(backup.read_text(encoding="utf-8"), current)
        self.assertEqual(stat.S_IMODE(backup.stat().st_mode), 0o600)
        self.assertEqual(self.target.read_text(encoding="utf-8"), current)

    def test_partial_backup_failure_removes_incomplete_file(self) -> None:
        descriptor = self.open_directory()
        current = 'name = "private"\n'
        self.target.write_text(current, encoding="utf-8")
        write = os.write

        def fail_after_copy(fd: int, data: bytes) -> int:
            write(fd, data[:3])
            raise OSError("injected write failure")

        with mock.patch.object(self.renderer.os, "write", side_effect=fail_after_copy):
            with self.assertRaisesRegex(OSError, "injected write failure"):
                self.renderer.backup_target(self.target, current, descriptor)
        self.assertEqual(self.backups(), [])
        self.assertEqual(self.target.read_text(encoding="utf-8"), current)

    def test_backup_fsync_failure_removes_incomplete_file(self) -> None:
        descriptor = self.open_directory()
        with mock.patch.object(self.renderer.os, "fsync", side_effect=OSError("fsync failed")):
            with self.assertRaisesRegex(OSError, "fsync failed"):
                self.renderer.backup_target(self.target, "private", descriptor)
        self.assertEqual(list(self.target.parent.iterdir()), [])

    def test_exclusive_creation_does_not_overwrite_or_chmod_existing_file(self) -> None:
        descriptor = self.open_directory()
        existing = self.target.parent / "existing.bak"
        existing.write_text("preserve", encoding="utf-8")
        existing.chmod(0o644)
        with self.assertRaises(FileExistsError):
            self.renderer.write_private_file(descriptor, existing.name, "replacement")
        self.assertEqual(existing.read_text(encoding="utf-8"), "preserve")
        self.assertEqual(stat.S_IMODE(existing.stat().st_mode), 0o644)

    def test_private_writer_completes_short_writes(self) -> None:
        descriptor = self.open_directory()
        write = os.write
        with mock.patch.object(self.renderer.os, "write", side_effect=lambda fd, data: write(fd, data[:2])):
            self.renderer.write_private_file(descriptor, "short.bak", "complete contents")
        self.assertEqual((self.target.parent / "short.bak").read_text(), "complete contents")

    def test_private_writer_rejects_zero_byte_write(self) -> None:
        descriptor = self.open_directory()
        with mock.patch.object(self.renderer.os, "write", return_value=0):
            with self.assertRaises(OSError):
                self.renderer.write_private_file(descriptor, "zero.bak", "contents")
        self.assertEqual(list(self.target.parent.iterdir()), [])

    def test_failed_replace_preserves_target_and_removes_temporary_file(self) -> None:
        descriptor = self.open_directory()
        self.target.write_text("original", encoding="utf-8")
        with mock.patch.object(self.renderer.os, "replace", side_effect=OSError("replace failed")):
            with self.assertRaisesRegex(OSError, "replace failed"):
                self.renderer.atomic_write(self.target, "new", descriptor)
        self.assertEqual(self.target.read_text(), "original")
        self.assertEqual(list(self.target.parent.iterdir()), [self.target])

    def test_permission_repair_rejects_a_replaced_symlink(self) -> None:
        descriptor = self.open_directory()
        self.target.write_text(self.template, encoding="utf-8")
        metadata = self.target.stat()
        self.target.unlink()
        outside = self.root / "outside.toml"
        outside.write_text(self.template, encoding="utf-8")
        outside.chmod(0o644)
        self.target.symlink_to(outside)
        with self.assertRaises(OSError):
            self.renderer.repair_permissions(descriptor, metadata)
        self.assertEqual(stat.S_IMODE(outside.stat().st_mode), 0o644)

    def test_directory_descriptor_does_not_follow_later_redirection(self) -> None:
        descriptor = self.open_directory()
        original = self.home / "original-agents"
        self.target.parent.rename(original)
        outside = self.root / "outside-agents"
        outside.mkdir()
        self.target.parent.symlink_to(outside, target_is_directory=True)
        self.renderer.atomic_write(self.target, "new", descriptor)
        self.assertEqual(list(outside.iterdir()), [])
        self.assertEqual((original / PROFILE_NAME).read_text(), "new")

    def test_backup_preserves_original_line_endings(self) -> None:
        self.target.parent.mkdir(parents=True)
        original = b'name = "original"\r\n# Preserve CRLF\r\n'
        self.target.write_bytes(original)
        self.run_renderer()
        self.assertEqual(self.backups()[0].read_bytes(), original)

    def test_render_rejects_a_replacement_inside_instructions(self) -> None:
        template = (
            'developer_instructions = """\nmodel = "example"\n"""\n'
            "model = 'actual-model'\n"
            'model_reasoning_effort = "high"\n'
        )
        for requested in ("replacement", "actual-model"):
            with self.subTest(requested=requested):
                with self.assertRaisesRegex(ValueError, "top-level"):
                    self.renderer.render(template, requested, "low")

    def test_render_rejects_nested_assignment_replacement(self) -> None:
        template = (
            "model = 'original'\nmodel_reasoning_effort = \"high\"\n"
            '[example]\nmodel = "example"\n'
        )
        with self.assertRaisesRegex(ValueError, "top-level"):
            self.renderer.render(template, "requested", "low")

    def test_render_changes_only_the_requested_top_level_fields(self) -> None:
        rendered = self.renderer.render(self.template, "test-worker", "minimal")
        self.assertEqual(
            tomllib.loads(rendered),
            {**self.defaults, "model": "test-worker", "model_reasoning_effort": "minimal"},
        )


class OpenAIMetadataTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        spec = importlib.util.spec_from_file_location("check_skills", ROOT / "scripts" / "check_skills.py")
        if spec is None or spec.loader is None:
            raise RuntimeError("cannot load the repository skill validator")
        cls.validator = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.validator)

    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.skill = Path(temporary.name) / "example-skill"
        self.path = self.skill / "agents" / "openai.yaml"
        self.path.parent.mkdir(parents=True)
        self.interface = (
            'interface:\n  display_name: "Example Skill"\n'
            '  short_description: "Exercise nested metadata validation"\n'
            '  default_prompt: "Use $example-skill for this task."\n'
        )

    def validate(self, extra: str = "") -> list[str]:
        self.path.write_text(self.interface + extra, encoding="utf-8")
        return self.validator.validate_openai_metadata(self.skill)

    def test_optional_sections_can_be_absent(self) -> None:
        self.assertEqual(self.validate(), [])

    def test_canonical_boolean_policies_are_accepted(self) -> None:
        for value in ("true", "false"):
            with self.subTest(value=value):
                self.assertEqual(self.validate(f"policy:\n  allow_implicit_invocation: {value}\n"), [])

    def test_non_boolean_policy_values_are_rejected(self) -> None:
        for value in ('"false"', "'true'", "yes", "on", "1", "null", "[]", "{}"):
            with self.subTest(value=value):
                errors = self.validate(f"policy:\n  allow_implicit_invocation: {value}\n")
                self.assertTrue(any("must be a boolean" in error for error in errors), errors)

    def test_policy_must_be_a_mapping(self) -> None:
        for value in ("false", "null", "[]", '"policy"'):
            with self.subTest(value=value):
                errors = self.validate(f"policy: {value}\n")
                self.assertTrue(any("'policy' must be a mapping" in error for error in errors), errors)

    def test_policy_typo_is_rejected(self) -> None:
        errors = self.validate("policy:\n  allow_implicit_invokation: false\n")
        self.assertTrue(any("unsupported policy fields" in error for error in errors), errors)

    def test_product_policy_accepts_supported_products(self) -> None:
        for products in ('["CHAT"]', '["CODEX"]', '["CHAT", "CODEX"]'):
            with self.subTest(products=products):
                self.assertEqual(self.validate(f"policy:\n  products: {products}\n"), [])

    def test_invalid_product_policy_is_rejected(self) -> None:
        for products in ('"CHAT"', "[]", '["OTHER"]', "[1]", "[{}]", "null"):
            with self.subTest(products=products):
                self.assertTrue(self.validate(f"policy:\n  products: {products}\n"))

    def test_dependencies_must_be_a_mapping(self) -> None:
        for value in ("null", "[]", '"mcp"'):
            with self.subTest(value=value):
                self.assertTrue(self.validate(f"dependencies: {value}\n"))

    def test_unknown_dependency_key_is_rejected(self) -> None:
        errors = self.validate("dependencies:\n  tool: []\n")
        self.assertTrue(any("unsupported dependencies fields" in error for error in errors), errors)

    def test_tools_must_be_a_list(self) -> None:
        for value in ("{}", "null", '"mcp"'):
            with self.subTest(value=value):
                self.assertTrue(self.validate(f"dependencies:\n  tools: {value}\n"))

    def test_valid_mcp_dependencies_and_empty_list_are_accepted(self) -> None:
        for tools in (
            "[]",
            '[{type: "mcp", value: "github"}]',
            ('[{type: "mcp", value: "github", description: "Repository access", '
             'transport: "streamable_http", url: "https://example.invalid/mcp"}]'),
        ):
            with self.subTest(tools=tools):
                self.assertEqual(self.validate(f"dependencies:\n  tools: {tools}\n"), [])

    def test_invalid_tool_entries_are_rejected(self) -> None:
        for tools in (
            '["mcp"]', '[{}]', '[{type: "mcp"}]', '[{value: "github"}]',
            '[{type: "other", value: "github"}]', '[{type: "mcp", value: 123}]',
            '[{type: "mcp", value: ""}]', '[{type: "mcp", value: "github", url: []}]',
            '[{type: "mcp", value: "github", description: false}]',
            '[{type: "mcp", value: "github", transport_type: "streamable_http"}]',
        ):
            with self.subTest(tools=tools):
                self.assertTrue(self.validate(f"dependencies:\n  tools: {tools}\n"))

    def test_duplicate_keys_are_rejected_at_every_level(self) -> None:
        for extra in (
            "policy: {}\npolicy: {}\n",
            "policy:\n  allow_implicit_invocation: true\n  allow_implicit_invocation: false\n",
            'dependencies:\n  tools: [{type: "mcp", value: "one", value: "two"}]\n',
        ):
            with self.subTest(extra=extra):
                errors = self.validate(extra)
                self.assertTrue(any("unique strings" in error for error in errors), errors)

    def test_non_string_keys_are_rejected(self) -> None:
        errors = self.validate("policy:\n  123: false\n")
        self.assertTrue(any("unique strings" in error for error in errors), errors)

    def test_unsafe_yaml_tags_are_rejected(self) -> None:
        errors = self.validate('policy: !!python/object/apply:os.system ["false"]\n')
        self.assertTrue(any("invalid YAML" in error for error in errors), errors)

    def test_metadata_interface_scalar_types_are_preserved(self) -> None:
        self.interface = self.interface.replace('"Example Skill"', "123")
        errors = self.validate()
        self.assertTrue(any("display_name" in error for error in errors), errors)



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
