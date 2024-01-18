# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Bash Select (Make Menu) https://linuxize.com/post/bash-select/

echo ""
echo "--------------------------------"
echo "Select number of the example to run: "
echo "--------------------------------"
echo "base: Basic Data Stream Example"
echo "quit: Exit"
echo "--------------------------------"
echo ""

select opt in  base quit;
do
  case $opt in

    base)
      echo "Selected example: Basic QD Client Example"
      command cargo run --release --bin basic_data_stream
      break
      ;;

    quit)
      echo "Exiting!"
      exit 0
      ;;

    *)
      echo "Invalid option $REPLY"
      ;;
  esac
done