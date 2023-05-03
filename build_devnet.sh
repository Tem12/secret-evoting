#!/bin/sh
make build
docker run --rm -v "$(pwd)":/contract \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  enigmampc/secret-contract-optimizer
#secretcli tx compute store contract.wasm.gz --gas 5000000 --from myWallet --chain-id secretdev-1 -y
