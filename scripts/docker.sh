# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Copy a temporary Dockerfile to the root directory
command cp flv_specs/service_specs/src/services/qdgw/Dockerfile Dockerfile_qdgw
# Build the image with the Docker deamon.
command docker build -t qdgw:latest -f Dockerfile_qdgw .
# Remove temporary Dockerfile
command rm Dockerfile_qdgw
