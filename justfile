set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    taplo fmt
    rumdl fmt .

clippy:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

check:
    cargo check --workspace --all-features --locked

test:
    cargo test --workspace --all-features --all-targets --locked

check-skills:
    uv run --script scripts/check_skills.py

cov:
    cargo llvm-cov --workspace --all-features --all-targets

ci: fmt check clippy test check-skills cov
    cargo machete
