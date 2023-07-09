#!/bin/bash

for i in {1..4000}
do
   echo "num: $i"
   secretcli tx compute execute secret120wmecer3xswzcj0hzuh9ldnmunyaxecvaj208 '{"submit_vote": {"candidate_id": 0}}' --from "dev_1" -y
done
