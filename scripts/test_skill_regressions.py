#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Exercise profile concurrency and marked examples without live agent calls."""

from __future__ import annotations

import json
import os
import re
import select
import shutil
import stat
import subprocess
import sys
import tempfile
import textwrap
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKER = ROOT / "skills" / "use-subagents-codex"
RENDERER = WORKER / "scripts" / "configure_worker_profile.py"
PROFILE_NAME = "use-subagents-codex.toml"
LOCK_NAME = ".use-subagents-codex.lock"
GENERICS = "skills/rust-best-practices/references/generics-static-and-dynamic-dispatch.md"
TESTING = "skills/rust-test/patterns/automated-testing.md"


@unittest.skipUnless(os.name == "posix", "installer requires POSIX permissions")
class WorkerConcurrencyTests(unittest.TestCase):
    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.home = self.root / "codex"
        self.target = self.home / "agents" / PROFILE_NAME
        self.lock = self.target.parent / LOCK_NAME
        self.template = (WORKER / "assets" / PROFILE_NAME).read_text(encoding="utf-8")
        self.environment = {
            **os.environ,
            "HOME": str(self.root / "user"),
            "CODEX_HOME": str(self.home),
            "PYTHONDONTWRITEBYTECODE": "1",
        }

    def command(self, *arguments: str) -> list[str]:
        return [sys.executable, str(RENDERER), "--codex-home", str(self.home), *arguments]

    def run_renderer(self, *arguments: str, expected_code: int = 0) -> subprocess.CompletedProcess[str]:
        result = subprocess.run(
            self.command(*arguments), capture_output=True, text=True,
            env=self.environment, timeout=10, check=False,
        )
        self.assertEqual(result.returncode, expected_code, result.stdout + result.stderr)
        return result

    def seed_target(self, content: str | None = None) -> None:
        self.target.parent.mkdir(parents=True, exist_ok=True)
        self.target.write_text(content if content is not None else self.template, encoding="utf-8")
        self.target.chmod(0o600)

    def test_dry_runs_never_create_a_lock_or_repair_permissions(self) -> None:
        self.run_renderer("--dry-run")
        self.assertFalse(self.home.exists())
        self.seed_target()
        self.target.chmod(0o644)
        before = self.target.read_bytes()
        self.run_renderer("--dry-run")
        self.run_renderer("--model", "test-other", "--dry-run")
        self.assertEqual(self.target.read_bytes(), before)
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o644)
        self.assertEqual(list(self.target.parent.iterdir()), [self.target])

    def test_lock_is_private_and_retains_its_inode_across_updates(self) -> None:
        self.run_renderer()
        before = self.lock.stat()
        self.assertEqual(stat.S_IMODE(before.st_mode), 0o600)
        self.assertEqual(self.lock.read_bytes(), b"")
        self.run_renderer("--model", "test-other")
        self.run_renderer("--model", "test-other")
        after = self.lock.stat()
        self.assertEqual((before.st_dev, before.st_ino), (after.st_dev, after.st_ino))
        self.assertEqual(stat.S_IMODE(after.st_mode), 0o600)

    def test_concurrent_install_rejects_conflict_and_retry_preserves_intermediate(self) -> None:
        original = 'name = "original"\nmodel = "test-original"\n'
        self.seed_target(original)
        config = self.home / "config.toml"
        parent = b'model = "parent"\nmodel_reasoning_effort = "low"\n'
        config.write_bytes(parent)
        ready_read, ready_write = os.pipe()
        release_read, release_write = os.pipe()
        # Pause process A inside its transaction. Pipes establish ordering without
        # sleeps; process B must not succeed while A owns the reconciliation lock.
        driver = textwrap.dedent('''
            import importlib.util, os, sys
            path, ready, release, home = sys.argv[1:]
            spec = importlib.util.spec_from_file_location("worker", path)
            worker = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(worker)
            original_backup = worker.backup_target
            def paused_backup(*args):
                os.write(int(ready), b"1")
                if os.read(int(release), 1) != b"1":
                    raise RuntimeError("test parent did not release installer")
                return original_backup(*args)
            worker.backup_target = paused_backup
            sys.argv = [path, "--codex-home", home, "--model", "test-worker-a"]
            raise SystemExit(worker.main())
        ''')
        process = None
        try:
            process = subprocess.Popen(
                [sys.executable, "-c", driver, str(RENDERER), str(ready_write),
                 str(release_read), str(self.home)],
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                env=self.environment, pass_fds=(ready_write, release_read),
            )
            os.close(ready_write)
            ready_write = -1
            os.close(release_read)
            release_read = -1
            readable, _, _ = select.select([ready_read], [], [], 10)
            self.assertTrue(readable, "first installer did not reach backup")
            self.assertEqual(os.read(ready_read, 1), b"1")
            result = self.run_renderer("--model", "test-worker-b", expected_code=1)
            self.assertIn("already in progress", result.stderr)
            self.assertEqual(self.target.read_text(), original)
            # Even a concurrent dry run remains read-only and does not block.
            lock_before = self.lock.stat()
            self.run_renderer("--model", "test-worker-b", "--dry-run")
            self.assertEqual(self.lock.stat().st_ctime_ns, lock_before.st_ctime_ns)
            self.assertEqual(list(self.target.parent.glob(f"{PROFILE_NAME}.bak.*")), [])
            os.write(release_write, b"1")
            output, error = process.communicate(timeout=10)
            self.assertEqual(process.returncode, 0, output + error)
        finally:
            if process is not None and process.poll() is None:
                process.kill()
                process.communicate(timeout=10)
            for descriptor in (ready_read, ready_write, release_read, release_write):
                if descriptor >= 0:
                    os.close(descriptor)
        intermediate = self.target.read_text(encoding="utf-8")
        self.assertEqual(tomllib.loads(intermediate)["model"], "test-worker-a")
        self.run_renderer("--model", "test-worker-b")
        backups = list(self.target.parent.glob(f"{PROFILE_NAME}.bak.*"))
        self.assertEqual(len(backups), 2)
        self.assertCountEqual([path.read_text() for path in backups], [original, intermediate])
        self.assertEqual(tomllib.loads(self.target.read_text())["model"], "test-worker-b")
        self.assertEqual(config.read_bytes(), parent)
        for path in [self.target, self.lock, *backups]:
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o600)

    def test_symlinked_locks_are_rejected_without_touching_referents(self) -> None:
        self.seed_target()
        outside = self.root / "outside"
        for exists in (True, False):
            with self.subTest(exists=exists):
                if exists:
                    outside.write_text("preserve", encoding="utf-8")
                    outside.chmod(0o644)
                self.lock.symlink_to(outside)
                self.run_renderer("--model", "test-other", expected_code=1)
                self.assertTrue(self.lock.is_symlink())
                self.assertEqual(self.target.read_text(), self.template)
                if exists:
                    self.assertEqual(outside.read_text(), "preserve")
                    self.assertEqual(stat.S_IMODE(outside.stat().st_mode), 0o644)
                    outside.unlink()
                else:
                    self.assertFalse(outside.exists())
                self.lock.unlink()

    def test_hardlinked_lock_is_rejected_without_chmod(self) -> None:
        self.seed_target()
        outside = self.root / "outside"
        outside.write_text("preserve", encoding="utf-8")
        outside.chmod(0o644)
        os.link(outside, self.lock)
        self.run_renderer("--model", "test-other", expected_code=1)
        self.assertEqual(outside.read_text(), "preserve")
        self.assertEqual(stat.S_IMODE(outside.stat().st_mode), 0o644)
        self.assertEqual(self.target.read_text(), self.template)

    def test_special_lock_paths_are_rejected_without_blocking(self) -> None:
        self.seed_target()
        os.mkfifo(self.lock)
        self.run_renderer("--model", "test-other", expected_code=1)
        self.assertTrue(stat.S_ISFIFO(self.lock.lstat().st_mode))
        self.lock.unlink()
        self.lock.mkdir()
        self.run_renderer("--model", "test-other", expected_code=1)
        self.assertTrue(self.lock.is_dir())
        self.assertEqual(self.target.read_text(), self.template)

    def test_permission_repair_obeys_the_same_lock(self) -> None:
        import fcntl

        self.seed_target()
        self.target.chmod(0o644)
        before = self.target.stat().st_mtime_ns
        with self.lock.open("w") as lock:
            fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
            result = self.run_renderer(expected_code=1)
            self.assertIn("already in progress", result.stderr)
            self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o644)
        self.run_renderer()
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o600)
        self.assertEqual(self.target.stat().st_mtime_ns, before)
        self.assertEqual(list(self.target.parent.glob(f"{PROFILE_NAME}.bak.*")), [])

    def test_failed_reconciliation_releases_lock(self) -> None:
        self.seed_target()
        driver = textwrap.dedent('''
            import importlib.util, sys
            path, home = sys.argv[1:]
            spec = importlib.util.spec_from_file_location("worker", path)
            worker = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(worker)
            def fail_replace(*args):
                raise OSError("injected replacement failure")
            worker.atomic_write = fail_replace
            sys.argv = [path, "--codex-home", home, "--model", "test-other"]
            try:
                worker.main()
            except OSError as error:
                print(error, file=sys.stderr)
                raise SystemExit(1)
        ''')
        result = subprocess.run(
            [sys.executable, "-c", driver, str(RENDERER), str(self.home)],
            env=self.environment, capture_output=True, text=True, timeout=10, check=False,
        )
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("injected replacement failure", result.stderr)
        self.assertEqual(self.target.read_text(), self.template)
        self.run_renderer("--model", "test-other")
        self.assertEqual(tomllib.loads(self.target.read_text())["model"], "test-other")


def example_block(relative_path: str, name: str, language: str) -> str:
    """Read the marked fence itself so tests cannot drift from the documentation."""
    text = (ROOT / relative_path).read_text(encoding="utf-8")
    marker = f"<!-- skill-example: {name} -->"
    if text.count(marker) != 1:
        raise ValueError(f"{relative_path}: expected exactly one {marker}")
    following = text.split(marker, 1)[1].lstrip()
    match = re.match(rf"```{re.escape(language)}\n(.*?)\n```(?:\n|$)", following, re.DOTALL)
    if match is None:
        raise ValueError(f"{relative_path}: {name} must precede a {language} fence")
    return match.group(1) + "\n"


def example_sources() -> tuple[str, str]:
    dependencies = example_block(TESTING, "insta-dependencies", "toml")
    concat = example_block(GENERICS, "concat-cloned", "rust")
    redaction = example_block(TESTING, "insta-json-redactions", "rust")
    manifest = '''[package]
name = "skill-example-fixture"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]

''' + dependencies
    tests = concat + '''
#[test]
fn concatenation_preserves_input_order_and_duplicates() {
    assert_eq!(concat_cloned(&[1, 3], &[2, 4]), vec![1, 3, 2, 4]);
    assert_eq!(concat_cloned(&[2, 2], &[2]), vec![2, 2, 2]);
    assert_eq!(concat_cloned::<u8>(&[], &[]), Vec::<u8>::new());
    assert_eq!(concat_cloned(&[], &[1]), vec![1]);
    assert_eq!(concat_cloned(&[1], &[]), vec![1]);
}

#[test]
fn concatenation_needs_clone_not_ord() {
    #[derive(Clone, Debug, PartialEq)]
    struct Value(u8);
    assert_eq!(concat_cloned(&[Value(3)], &[Value(1)]), vec![Value(3), Value(1)]);
}

#[test]
fn json_redactions_match_the_documented_example() {
    let job_payload = std::collections::BTreeMap::from([
        ("finished_at", "test-timestamp"),
        ("run_id", "test-run-id"),
        ("status", "complete"),
    ]);
''' + textwrap.indent(redaction, "    ") + "}\n"
    return manifest, tests


class ExampleSourceTests(unittest.TestCase):
    def test_marked_examples_and_manifest_can_be_extracted(self) -> None:
        manifest, tests = example_sources()
        parsed = tomllib.loads(manifest)
        self.assertEqual(parsed["package"]["edition"], "2024")
        self.assertEqual(set(parsed["dev-dependencies"]["insta"]["features"]),
                         {"yaml", "json", "redactions"})
        self.assertIn(example_block(GENERICS, "concat-cloned", "rust"), tests)
        self.assertIn(textwrap.indent(example_block(TESTING, "insta-json-redactions", "rust"), "    "), tests)

    def test_missing_example_fails_instead_of_silently_skipping(self) -> None:
        with self.assertRaisesRegex(ValueError, "exactly one"):
            example_block(TESTING, "missing-example", "rust")


class RustExampleTests(unittest.TestCase):
    def test_documented_examples_compile_and_preserve_their_contracts(self) -> None:
        cargo = shutil.which("cargo")
        self.assertIsNotNone(cargo, "Cargo is required; use ExampleSourceTests for extraction-only checks")
        manifest, tests = example_sources()
        with tempfile.TemporaryDirectory(prefix="skill-examples-") as directory:
            root = Path(directory)
            (root / "src").mkdir()
            (root / "tests").mkdir()
            (root / "src" / "lib.rs").write_text("", encoding="utf-8")
            (root / "Cargo.toml").write_text(manifest, encoding="utf-8")
            (root / "tests" / "examples.rs").write_text(tests, encoding="utf-8")
            environment = {
                **os.environ,
                "CARGO_TARGET_DIR": str(root / "target"),
                "RUSTC_WRAPPER": "",
                "RUSTC_WORKSPACE_WRAPPER": "",
                # Only this temporary fixture may create/accept snapshots.
                "INSTA_UPDATE": "always",
                "INSTA_FORCE_PASS": "0",
            }
            result = subprocess.run(
                [cargo, "test", "--manifest-path", str(root / "Cargo.toml"), "--test", "examples"],
                cwd=root, env=environment, capture_output=True, text=True,
                timeout=300, check=False,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            snapshots = list((root / "tests").rglob("*.snap"))
            self.assertEqual(len(snapshots), 1, result.stdout + result.stderr)
            body = snapshots[0].read_text(encoding="utf-8").split("---\n", 2)[-1]
            self.assertEqual(json.loads(body), {
                "finished_at": "[timestamp]", "run_id": "[run-id]", "status": "complete",
            })


if __name__ == "__main__":
    unittest.main()
