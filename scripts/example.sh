# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Bash Select (Make Menu) https://linuxize.com/post/bash-select/

echo ""
echo "-----------------------------------------"
echo "Select the number of the example to run: "
echo "-----------------------------------------"
echo "1) Base: Basic Data Stream Example. Requires QDGW running"
echo "2) CausalModel: Real-Time Causal Inference Example. Requires QDGW running"
echo "3) SymbolMaster Example. Requires QDGW and SYMDB running"
echo "4) quit: Exit"
echo ""
echo "-----------------------------------------"
echo "Start  QDGW: make  qdgw"
echo "Start SYMDB: make  symdb"
echo "-----------------------------------------"
echo ""

select opt in  Base CausalModel SymbolMaster quit;
do
  case $opt in

    Base)
        echo "Selected example: Basic QD Client Example"
        command cargo run --bin basic_data_stream
        break
        ;;

    CausalModel)
        echo "Selected example: Causal Inference QD Client Example"
        command cargo run --bin causal_data_inference
        break
        ;;

    SymbolMaster)
        echo "Selected example: Symbol Master Example"
        command cargo run --bin symbol_master
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