# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# https://nexte.st/book/installing-from-source.html
# cargo install cargo-nextest --locked

command cargo test --doc

command cargo nextest run

# Once the project got more than 500 tests, the following commands most likely executes them way faster
# because of significantly better CPU optimizations. Run a quick comparison to see the difference.

# RUSTFLAGS='-C target-cpu=native' cargo test --doc --release
# RUSTFLAGS='-C target-cpu=native' cargo nextest run --release