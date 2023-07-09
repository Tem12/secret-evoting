#!/bin/bash

for i in {1..4000}
do
   VAL=$(secretcli keys add "dev_$i" | jq -r '.address')
   echo $VAL >> addr_test.txt
   echo "wallet: $i"
   curl "http://localhost:5000/faucet?address=$VAL"
done
