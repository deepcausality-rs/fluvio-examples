# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command  cargo run --bin qdgw
