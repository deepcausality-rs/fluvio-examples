# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cargo fix --lib --allow-dirty

command cargo clippy --fix --allow-dirty