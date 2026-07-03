set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    taplo fmt
    rumdl fmt .

clippy:
    cargo clippy --workspace --all-features --all-targets -- -D warnings

check:
    cargo check --workspace --all-features

test:
    cargo test --workspace --all-features --all-targets

cov:
    cargo llvm-cov --workspace --all-features --all-targets

ci: fmt check clippy test cov
    cargo machete
