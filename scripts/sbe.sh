# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Check if Java is working
# Java is assumed to be installed with SDKMAN
# https://www.andrewhoog.com/post/3-ways-to-install-java-on-macos-2023/#install-java-with-sdkman-1
command java --version >/dev/null 2>&1 || {
#  If not, source it.
    source "$HOME/.sdkman/bin/sdkman-init.sh"
}

#  https://github.com/real-logic/simple-binary-encoding?tab=readme-ov-file
command java -Dsbe.generate.ir=true -Dsbe.target.language=Rust -Dsbe.target.namespace=sbe -Dsbe.output.dir=queng_sbe/ -Dsbe.errorLog=yes -jar tools/sbe/sbe-all-1.30.0.jar queng_sbe/sbe_schema/schema.xml

echo "Done: SBE Bindings generated!"
exit 0
