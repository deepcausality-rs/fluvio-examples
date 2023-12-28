# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Code formatting
# This prevents Clippy from reporting formatting errors
# https://github.com/rust-lang/rustfmt
command cargo fmt --all


# Check for outdated dependencies
# https://github.com/kbknapp/cargo-outdated
command cargo outdated --workspace


# Scan for unused dependencies
# https://crates.io/crates/cargo-udeps
command cargo +nightly udeps --all-targets


# Scan again to report all unfixed vulnerabilities
# https://crates.io/crates/cargo-audit
command cargo audit


# Check a package and all of its dependencies for errors.
# https://doc.rust-lang.org/cargo/commands/cargo-check.html
command cargo check --all-targets

# Consider checking each crate for re-exporting external types
# https://crates.io/crates/cargo-check-external-types
# cargo +nightly check-external-types


# Check for linter errors
# https://github.com/rust-lang/rust-clippy
command cargo clippy --all-targets


# Check code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all --check
