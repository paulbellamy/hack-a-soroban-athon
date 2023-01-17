#!/bin/bash

set -e

# TODO: Set the recipient to something reasonable here. Probably whatever account
# soroban is running stuff as?
# TODO: Have a nicer way to build Identifiers on the CLI
TOKEN_ADMIN="GDT2NORMZF6S2T4PT4OBJJ43OPD3GPRNTJG3WVVFB356TUHWZQMU6C3U"
TOKEN_ADMIN_IDENTIFIER="AAAABAAAAAEAAAAAAAAAAgAAAAUAAAAHQWNjb3VudAAAAAAEAAAAAQAAAAgAAAAA56a6LMl9LU+PnxwUp5tzx7M+LZpNu1alDvvp0PbMGU8="

case "$1" in
standalone)
  echo "Using standalone network"
  export SOROBAN_RPC_HOST="http://localhost:8000"
  export SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"
  export SOROBAN_NETWORK_PASSPHRASE="Standalone Network ; February 2017"
  export SOROBAN_SECRET_KEY="SAKCFFFNCE7XAWYMYVRZQYKUK6KMUCDIINLWISJYTMYJLNR2QLCDLFVT"

  echo Fund token admin account from friendbot
  curl "$SOROBAN_RPC_HOST/friendbot?addr=$TOKEN_ADMIN"
  ;;
futurenet)
  echo "Using Futurenet network"
  export SOROBAN_RPC_HOST="http://localhost:8000"
  export SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"
  export SOROBAN_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
  export SOROBAN_SECRET_KEY="SAKCFFFNCE7XAWYMYVRZQYKUK6KMUCDIINLWISJYTMYJLNR2QLCDLFVT"
  # TODO: Use friendbot to fund the token admin, or figure our token admin here...
  curl "https://friendbot-futurenet.stellar.org/?addr=$TOKEN_ADMIN"
  ;;
""|sandbox)
  # no-op
  ;;
*)
  echo "Usage: $0 sandbox|standalone|futurenet"
  exit 1
  ;;
esac


echo Wrap the Stellar asset for the quest badges to ensure it is accessible from soroban
mkdir -p .soroban
TOKEN_ID=$(soroban token wrap --asset "EXT:$TOKEN_ADMIN")
echo -n "$TOKEN_ID" > .soroban/token_id

echo Build the voting contract
make build

echo Deploy the voting contract
VOTING_ID="$(
  soroban deploy \
    --wasm target/wasm32-unknown-unknown/release/soroban_voting_contract.wasm
)"
echo "$VOTING_ID" > .soroban/voting_id

echo "Contract deployed succesfully with ID: $VOTING_ID"

echo "Initialize the voting contract"
# soroban invoke \
#   --id "$VOTING_ID" \
#   --fn initialize \
#   --arg-xdr "$TOKEN_ADMIN_IDENTIFIER" \
#   --wasm target/wasm32-unknown-unknown/release/soroban_voting_contract.wasm

echo "Done"
