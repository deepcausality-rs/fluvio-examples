# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cargo test --doc

command cargo doc --no-deps --workspace --open