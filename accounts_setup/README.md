# Accounts Setup script

This script is used to setup the accounts on the server. It is used to create the accounts, the asset and send the asset
to the accounts.

## Install

```bash
$ cd accounts_setup
$ pipenv install
```

## Activane the pipenv environment

```bash
$ source ~/.local/share/virtualenvs/{accounts_setup}/bin/activate  # replace {accounts_setup} with your pipenv environment path
```

## Usage

Help can be found by running the script with the `-h` flag.

```bash
$ pipenv run python script.py -h

Sets up the receiver accounts with the desired asset

options:
  -h, --help            show this help message and exit
  -l SECRETS_LIST [SECRETS_LIST ...], --secrets-list SECRETS_LIST [SECRETS_LIST ...]
                        the list of secrets we want to setup
  -s ASSET_ISSUER_SECRET, --asset-issuer-secret ASSET_ISSUER_SECRET
                        the secret of the asset issuer
  -c ASSET_CODE, --asset-code ASSET_CODE
                        the code of the asset we want to setup
  -i ASSET_ISSUER, --asset-issuer ASSET_ISSUER
                        the issuer of the asset we want to setup
  -p HORIZON_PASSPHRASE, --horizon-passphrase HORIZON_PASSPHRASE
                        the passphrase of the horizon network to use
  -u HORIZON_URL, --horizon-url HORIZON_URL
                        the horizon url to use
  -f FRIENDBOT_URL, --friendbot-url FRIENDBOT_URL
                        the friendbot url available for the network.
```

## Example

```bash
pipenv run python script.py \ 
  --secrets-list SCRBE7KIGIOH7W5CQA2OTZ2D2TQPX6RR2WTFFAV347BLOIZU5BD5VTHN \
  --asset-issuer-secret SC4OVVO2RKSUXWAPRMOQSHDZL2Q5COBGOFZSF5MWVTYXC2LHVFFSAVEJ \
  --asset-code=MYNFT \
  --asset-issuer=GBHCFFI5ZCRNHW6RZVR5WG7ERSP7NOOVKF36QBB4PVESDCMZQL6OQYEM

✅ Issuer account is ready: GBHCFFI5ZCRNHW6RZVR5WG7ERSP7NOOVKF36QBB4PVESDCMZQL6OQYEM
✅ Receiver accounts are ready: ['GD4M6VSOW5SJI5QQ6BTYGQ4EKM774H3E3LMLH4J3VNDVISMO33DF577G']
✅ Asset is ready in the accounts balances!
🎉 Done! The following accounts have a balance of at least 0.0000001 of the asset: <Asset [code=MYNFT, issuer=GBHCFFI5ZCRNHW6RZVR5WG7ERSP7NOOVKF36QBB4PVESDCMZQL6OQYEM, type=credit_alphanum12]>
   - GD4M6VSOW5SJI5QQ6BTYGQ4EKM774H3E3LMLH4J3VNDVISMO33DF577G
```