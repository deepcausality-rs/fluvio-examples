# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Ensure JAVA is in path
#command  source "$HOME/.sdkman/bin/sdkman-init.sh"


# Generate SBE Bindings for Rust using the SBE tool
#  https://github.com/real-logic/simple-binary-encoding?tab=readme-ov-file
command java -Dsbe.generate.ir=true -Dsbe.target.language=Rust -Dsbe.target.namespace=sbe -Dsbe.output.dir=flv_sbe/ -Dsbe.errorLog=yes -jar tools/sbe/sbe-all-1.30.0.jar flv_sbe/sbe_schema/schema.xml
echo "Done: SBE Bindings generated!"
exit 0
