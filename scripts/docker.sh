# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command cp queng_specs/service_specs/src/services/cmdb/Dockerfile Dockerfile_cmdb
command docker build -t cmdb:latest -f Dockerfile_cmdb .
command rm Dockerfile_cmdb
