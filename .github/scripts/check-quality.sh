#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all -- --check
cargo sort-derives --check
taplo fmt --check
rumdl check .
typos
actionlint
shellcheck .github/scripts/*.sh
git diff --check
