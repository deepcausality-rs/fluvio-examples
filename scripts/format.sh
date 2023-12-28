# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all