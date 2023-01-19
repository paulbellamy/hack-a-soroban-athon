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

  ;;
futurenet)
  echo "Using Futurenet network"
  export SOROBAN_RPC_HOST="http://localhost:8000"
  export SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"
  export SOROBAN_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
  export SOROBAN_SECRET_KEY="$TOKEN_ADMIN_SECRET"
  ;;
sandbox)
  export SOROBAN_ACCOUNT="$TOKEN_ADMIN"
  # no-op
  ;;
*)
  echo "Usage: $0 sandbox|standalone|futurenet submission|voting|finished"
  exit 1
  ;;
esac

case "$2" in
submission)
  STATUS=0
  ;;
voting)
  STATUS=1
  ;;
finished)
  STATUS=2
  ;;
*)
  echo "Usage: $0 sandbox|standalone|futurenet submission|voting|finished"
  exit 1
  ;;
esac

soroban invoke \
  --id $(cat .soroban/voting_id) \
  --fn set_status \
  --arg $STATUS

echo "Done"
