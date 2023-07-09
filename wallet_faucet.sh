#!/bin/bash

File="addr_test.txt"
Lines=$(cat $File)
for Line in $Lines
do
  echo "$Line"
  curl "http://localhost:5000/faucet?address=$Line"
done
