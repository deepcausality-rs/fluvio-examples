# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cargo clean

command rm -rf sbe/bindings
command rm sbe_hashes.md5
