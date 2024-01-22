# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Bash Select (Make Menu) https://linuxize.com/post/bash-select/

echo ""
echo "-----------------------------------------"
echo "Select the number of the example to run: "
echo "-----------------------------------------"
echo "1) base: Basic Data Stream Example"
echo "2) causal: Real-Time Causal Inference Example"
echo "3) quit: Exit"
echo "-----------------------------------------"
echo ""

select opt in  base causal quit;
do
  case $opt in

    base)
      echo "Selected example: Basic QD Client Example"
      command cargo run --release --bin basic_data_stream
      break
      ;;

    causal)
            echo "Selected example: Causal Inference QD Client Example"
            command cargo run --release --bin causal_data_inference
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