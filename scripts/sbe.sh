# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Check if Java is working
command java --version >/dev/null 2>&1 || {
  echo "ERROR: No JAVA detected. Install or configure JDK 17 or later!"
}


# Generate SBE Bindings for Rust using the SBE tool
#  https://github.com/real-logic/simple-binary-encoding?tab=readme-ov-file
command java -Dsbe.generate.ir=true -Dsbe.target.language=Rust -Dsbe.target.namespace=sbe -Dsbe.output.dir=sbe/ -Dsbe.errorLog=yes -jar tools/sbe/sbe-all-1.30.0.jar sbe/sbe_schema/schema.xml

echo "Done: SBE Bindings generated!"
exit 0
