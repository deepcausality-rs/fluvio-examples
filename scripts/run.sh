# bin/sh
set -o errexit
set -o nounset
set -o pipefail


#RUSTFLAGS='-C target-cpu=native'  cargo run --bin qdgw --release

cargo run --bin qdgw
