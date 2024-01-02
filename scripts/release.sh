# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# https://users.rust-lang.org/t/how-to-best-ensure-target-cpu-native/53167
RUSTFLAGS='-C target-cpu=native' cargo build --release