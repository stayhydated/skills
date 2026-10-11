#!/usr/bin/env bash
set -euo pipefail

# Keep versions explicit: formatter output and command flags are CI contracts.
tools_dir="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/ci-quality-tools"
mkdir -p "$tools_dir/bin"
cargo binstall --locked --no-confirm --root "$tools_dir" \
  taplo-cli@0.10.0 rumdl@0.2.78 cargo-sort-derives@0.13.0 \
  typos-cli@1.42.3

curl --fail --silent --show-error --location \
  https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_linux_amd64.tar.gz \
  --output "$tools_dir/actionlint.tar.gz"
printf '%s  %s\n' \
  8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8 \
  "$tools_dir/actionlint.tar.gz" | sha256sum --check
tar -xzf "$tools_dir/actionlint.tar.gz" -C "$tools_dir/bin" actionlint

curl --fail --silent --show-error --location \
  https://github.com/koalaman/shellcheck/releases/download/v0.11.0/shellcheck-v0.11.0.linux.x86_64.tar.xz \
  --output "$tools_dir/shellcheck.tar.xz"
printf '%s  %s\n' \
  8c3be12b05d5c177a04c29e3c78ce89ac86f1595681cab149b65b97c4e227198 \
  "$tools_dir/shellcheck.tar.xz" | sha256sum --check
tar -xJf "$tools_dir/shellcheck.tar.xz" -C "$tools_dir/bin" \
  --strip-components=1 shellcheck-v0.11.0/shellcheck

if [[ -n "${GITHUB_PATH:-}" ]]; then
  printf '%s\n' "$tools_dir/bin" >> "$GITHUB_PATH"
fi
