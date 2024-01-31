############################
# Build image
# This Dockerfile runs relative to the repo root.
# Therefore, copy it there before running.
############################
FROM rust:1.75-slim-buster as builder

# Create the user and group files to run the binary
# in the final scratch container as an unprivileged user.
RUN mkdir /user && \
    echo 'nobody:x:65534:65534:nobody:/:' > /user/passwd && \
    echo 'nobody:x:65534:' > /user/group

# Install protobuf compiler
# Install git to fix issue #3787 in fluvio
# https://github.com/infinyon/fluvio/issues/3787
RUN apt-get update && apt-get install -y git protobuf-compiler libprotobuf-dev

# Install build target musel for compiling a static binary
RUN rustup target add aarch64-unknown-linux-musl

# Set workspace directory
WORKDIR /app

# Copy over the entire source code
COPY . ./

# Fix linker issue with musel that affects both, AMD64 and ARM64 (aarch64).
# https://github.com/rust-lang/stacker/issues/80
ARG CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ARG CC=aarch64-linux-gnu-gcc

# Download all dependencies
RUN cargo fetch

# Because of Issue #10174 in SurrealDB, Docker requires at least 4GB of memory build the image.
# When memory is low, the docker build aborts after 15+ min with (signal: 9, SIGKILL: kill)
# Just set memory limit in Docker to at least 4GB, better 12GB, and the issue is gone.
# https://github.com/google/oss-fuzz/issues/10174

# Run the release build.
RUN RUSTFLAGS='-C target-cpu=native' cargo build -p qdgw --release --target aarch64-unknown-linux-musl

# Move binary up to root level directory for easy access
RUN mv /app/target/aarch64-unknown-linux-musl/release/qdgw /qdgw

############################
# Scratch image
############################
# Create minimal docker image
FROM scratch as runner

# Import user and group files from the build stage.
COPY --from=builder /user/group /user/passwd /etc/

# Import the CAcertificates from the build stage to enable HTTPS.
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy binary from build output directory
COPY --from=builder /services/qdgw /

# Port number must match deployment.yaml and specs/*/service_info
# Fluvio PORT: 9003 9005 9010
# metrics PORT: 8080
EXPOSE 9003 9005 9010 8080

# Run binary as unprivileged user
USER nobody:nobody

# Hard coded start command b/c no shell in scratch image
CMD ["/qdgw"]
