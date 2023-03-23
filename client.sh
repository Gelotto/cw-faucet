#!/bin/bash

CMD=$1
NETWORK=$2
NODE=
CHAIN_ID=

TAG=$3
if [ -z "$TAG" ]; then
  TAG=$(cat ./builds/latest)
fi

CONTRACT_ADDR=$(cat ./builds/build-$TAG/latest-contract)

shift 3

case $NETWORK in
  testnet)
    NODE="https://rpc.uni.juno.deuslabs.fi:443"
    CHAIN_ID=uni-3
    DENOM=ujunox
    ;;
  mainnet)
    NODE="https://rpc-juno.itastakers.com:443"
    CHAIN_ID=juno-1
    DENOM=ujuno
    ;;
  devnet)
    NODE="http://localhost:26657"
    CHAIN_ID=testing
    DENOM=ujunox
    ;;
esac

FLAGS="\
  --gas auto --gas-adjustment 1.5 --broadcast-mode block -o json -y \
  --node $NODE --gas-prices 0.025$DENOM --chain-id $CHAIN_ID"

execute() {
  sender=$1
  msg=$2
  flags="$FLAGS --from $sender"
  echo junod tx wasm execute $CONTRACT_ADDR "$msg" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" "$msg" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}


configure() {
  sender=$1
  interval=$2
  msg='{"configure":{"params":[{"interval":"'$interval'","token":{"native":{"denom":"'$DENOM'"}}}]}}'
  execute $sender $msg
}

transfer() {
  sender=$1
  recipient=$2
  amount=$3
  msg='{"transfer":{"recipient":"'$recipient'","tokens":[{"amount":"'$amount'","token":{"native":{"denom":"'$DENOM'"}}}]}}'
  execute $sender $msg
}

select-all() {
  sender=$1
  query='{"select":{"fields":null,"wallet":"'$sender'"}}'
  flags="--chain-id $CHAIN_ID --output json --node $NODE"
  echo junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags
  response=$(junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

set -e
echo "executing $CMD for $CONTRACT_ADDR"

case $CMD in
  transfer)
    transfer $1 $2 $3
    ;;
  configure)
    configure $1 $2
    ;;
  select) 
    select-all $1
    ;;
  *)
    echo "unrecognized option: $CMD" >&2
    exit -1
esac