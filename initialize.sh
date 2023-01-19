#!/bin/bash

set -e

# TODO: Set the recipient to something reasonable here. Probably whatever account
# soroban is running stuff as?
# TODO: Have a nicer way to build Identifiers on the CLI
export TOKEN_CODE="MYNFT"
export TOKEN_ADMIN="GBHCFFI5ZCRNHW6RZVR5WG7ERSP7NOOVKF36QBB4PVESDCMZQL6OQYEM"
TOKEN_ADMIN_HEX="4e22951dc8a2d3dbd1cd63db1be48c9ff6b9d55177e8043c7d4921899982fce8"
TOKEN_ADMIN_SECRET="SC4OVVO2RKSUXWAPRMOQSHDZL2Q5COBGOFZSF5MWVTYXC2LHVFFSAVEJ"

case "$1" in
standalone)
  echo "Using standalone network"
  export SOROBAN_RPC_HOST="http://localhost:8000"
  export SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"
  export SOROBAN_NETWORK_PASSPHRASE="Standalone Network ; February 2017"
  export SOROBAN_SECRET_KEY="$TOKEN_ADMIN_SECRET"

  echo Fund token admin account from friendbot
  curl "$SOROBAN_RPC_HOST/friendbot?addr=$TOKEN_ADMIN"
  ;;
futurenet)
  echo "Using Futurenet network"
  export SOROBAN_RPC_HOST="http://localhost:8000"
  export SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"
  export SOROBAN_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
  export SOROBAN_SECRET_KEY="$TOKEN_ADMIN_SECRET"
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
# TOKEN_ID=$(soroban token wrap --asset "$TOKEN_CODE:$TOKEN_ADMIN")
TOKEN_ID=$(soroban token wrap --asset "native")
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
soroban invoke \
  --id "$VOTING_ID" \
  --fn initialize \
  --arg "{\"object\":{\"vec\":[{\"symbol\":\"Account\"},{\"object\":{\"account_id\":{\"public_key_type_ed25519\":\"$TOKEN_ADMIN_HEX\"}}}]}}" \
  --arg "$TOKEN_ID" \
  --arg "1" \
  --wasm target/wasm32-unknown-unknown/release/soroban_voting_contract.wasm

echo "Done"
