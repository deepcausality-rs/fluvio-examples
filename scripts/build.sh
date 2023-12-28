# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cargo build

#command RUSTFLAGS="-Z threads=8" cargo +nightly build
