# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Better performance
RUSTFLAGS='-C target-cpu=native'  cargo run --bin qdgw --release

# Faster compile
#cargo run --bin qdgw
