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
RUN apt-get update && apt-get install -y git pkg-config libssl-dev

# Set workspace directory
WORKDIR /app

# Copy over the entire source code
COPY . ./

# Download all dependencies
RUN cargo fetch

# Build the release.
RUN #RUSTFLAGS='-C target-cpu=native' cargo build -p qdgw --release
RUN cargo build -p qdgw --release

# Move binary up to root level directory for easy access
RUN mv /app/target/release/qdgw /qdgw

############################
# Run image
############################
# Create minimal docker image
FROM alpine as runner

# Import user and group files from the build stage.
COPY --from=builder /user/group /user/passwd /etc/

# Import the CA-certificates from the build stage to enable HTTPS.
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy binary from build output directory
COPY --from=builder /qdgw /

# Port number must match deployment.yaml and specs/*/service_info
# Fluvio PORT: 9003 9005 9010
# metrics PORT: 8080
EXPOSE 9003 9005 9010 8080

# Run binary as unprivileged user
USER nobody:nobody

# Hard coded start command b/c no shell
CMD ["/qdgw"]
