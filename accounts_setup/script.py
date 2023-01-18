import argparse
from stellar_sdk import Keypair, Server, Asset, TransactionBuilder
from stellar_sdk.exceptions import NotFoundError
from urllib.request import urlopen, Request
from typing import List, Tuple


def parse_args():
    parser = argparse.ArgumentParser(
        prog='Account Setup',
        description='Sets up the receiver accounts with the desired asset'
    )

    # The asset receivers:
    parser.add_argument('-l', '--secrets-list', nargs='+', required=True,
                        help='the list of secrets we want to setup')
    # # The secret of the asset issuer:
    parser.add_argument('-s', '--asset-issuer-secret', type=str, required=True,
                        help='the secret of the asset issuer')
    # # The asset to setup:
    parser.add_argument('-c', '--asset-code', type=str, required=True,
                        help='the code of the asset we want to setup')
    parser.add_argument('-i', '--asset-issuer', type=str, required=True,
                        help='the issuer of the asset we want to setup')
    # Horizon args:
    parser.add_argument('-p', '--horizon-passphrase', type=str,
                        default='Test SDF Future Network ; October 2022',
                        help='the passphrase of the horizon network to use')
    parser.add_argument('-u', '--horizon-url', type=str,
                        default='https://horizon-futurenet.stellar.org',
                        help='the horizon url to use')
    parser.add_argument('-f', '--friendbot-url', type=str, required=False,
                        default='https://friendbot-futurenet.stellar.org/',
                        help='the friendbot url available for the network.')

    args = parser.parse_args()
    return args


def main():
    print("this is the main function")
    args = parse_args()

    horizon_server: Server = Server(horizon_url=args.horizon_url)
    network_passphrase: Server = args.horizon_passphrase
    friendbot_url: str = args.friendbot_url

    # Make sure issuer account is ready
    account_issuer_secret: str = args.asset_issuer_secret
    (issuer_kp, issuer_account_dict) = fund_accounts_if_needed(
        horizon_server=horizon_server,
        friendbot_url=friendbot_url,
        secrets=[account_issuer_secret],
    )[0]
    print(f"✅ Issuer account is ready: {issuer_kp.public_key}")

    # Make sure the receiver accounts are ready
    receiver_secrets: List[str] = args.secrets_list
    receivers_tuples: List[Tuple[Keypair, dict]] = fund_accounts_if_needed(
        horizon_server=horizon_server,
        friendbot_url=friendbot_url,
        secrets=receiver_secrets,
    )
    print(f"✅ Receiver accounts are ready: {[r[0].public_key for r in receivers_tuples]}")
    horizon_server.accounts().account_id(issuer_kp.public_key).call()

    # Setup the asset
    asset_code: str = args.asset_code
    asset_issuer: str = args.asset_issuer
    asset = Asset(code=asset_code, issuer=asset_issuer)
    prepare_accounts_for_asset(asset, horizon_server, network_passphrase, issuer_kp, receivers_tuples)
    print("✅ Asset is ready in the accounts balances!")

    print(f"🎉 Done! The following accounts have a balance of at least 0.0000001 of the asset: {asset}")
    for receiver in receivers_tuples:
        (receiver_kp, _) = receiver
        print(f"   - {receiver_kp.public_key}")


def prepare_accounts_for_asset(
    asset: Asset,
    horizon_server: Server,
    network_passphrase: str,
    issuer_kp: Keypair,
    receivers_tuples: List[Tuple[Keypair, dict]]
) -> None:
    "Make sure the receiver accounts have the trustline and enough balance on the asset."
    account_kps_to_receive_asset: List[Keypair] = []
    for receiver in receivers_tuples:
        (receiver_kp, receiver_account) = receiver
        if not contains_asset(account=receiver_account, asset=asset):
            print(f"Account {receiver_kp.public_key} does not have the asset {asset.code}!")
            print(f"Will send asset {asset.code} to {receiver_kp.public_key}.")
            account_kps_to_receive_asset.append(receiver_kp)

    if account_kps_to_receive_asset:
        print(f"Sending asset {asset.code} to {len(account_kps_to_receive_asset)} account(s).")
        issuer_account = horizon_server.load_account(account_id=issuer_kp.public_key)
        # Build the Stellar transaction:
        transaction = (
            TransactionBuilder(
                source_account=issuer_account,
                network_passphrase=network_passphrase,
                base_fee=1000000,
            )
            .set_timeout(30)
            # .append_change_trust_op(asset=asset, source=issuer_kp.public_key)
            # .build()
        )
        for receiver_kp in account_kps_to_receive_asset:
            transaction = transaction.append_change_trust_op(asset=asset, source=receiver_kp.public_key)
            transaction = transaction.append_payment_op(
                destination=receiver_kp.public_key,
                asset=asset,
                amount="0.0000001",
                source=issuer_kp.public_key,
            )
        transaction = transaction.build()

        # Sign the transaction:
        transaction.sign(issuer_kp)
        for receiver_kp in account_kps_to_receive_asset:
            transaction.sign(receiver_kp)

        # Submit the transaction:
        try:
            response = horizon_server.submit_transaction(transaction)
        except Exception as ex:
            print(f"Transaction submission failed: {ex}")
            exit(-1)
        else:
            print(f"Tansaction submissipon succeeded, hash: {response['hash']}")
            print(f"Successful?: {response.get('successful')}")


def contains_asset(account: dict, asset: Asset, min_balance: str = "0.0000001") -> bool:
    "Checks if the given account contains the given asset."
    balances: List[dict] = account.get('balances')
    if not balances:
        raise ValueError("Account does not have any balance!")

    for balance in balances:
        if all([
            balance.get("asset_issuer") == asset.issuer,
            balance.get("balance") > min_balance
        ]):
            return True


def fund_accounts_if_needed(
    horizon_server: Server, friendbot_url: str, secrets: List[str]
) -> List[Tuple[Keypair, dict]]:
    "Makes sure that the given accounts exist, if not, funds them."

    if not secrets:
        raise ValueError("No secrets provided.")

    keypairs: List[Keypair] = []
    for secret in secrets:
        try:
            keypairs.append(Keypair.from_secret(secret))
        except Exception:
            raise ValueError(f"Invalid secret provided: {secret}")

    tuple: Tuple[Keypair, dict] = []
    for kp in keypairs:
        try:
            account = horizon_server.accounts().account_id(kp.public_key).call()
            tuple.append((kp, account))
        except NotFoundError:
            print(f"Account {kp.public_key} does not exist! Funding it...")
            fund_stellar_account(
                friendbot_url=friendbot_url, public_key=kp.public_key
            )
            account = horizon_server.accounts().account_id(kp.public_key).call()
            tuple.append((kp, account))

    return tuple


def fund_stellar_account(friendbot_url: str, public_key: str) -> bool:
    "Funds a stellar account using the given frinedbot url."
    if not friendbot_url:
        raise ValueError("Friendbot url is not set!")

    if not public_key:
        raise ValueError("Public key is not set!")

    url = f"{friendbot_url}?addr={public_key}"
    headers = {"User-Agent": "Relief Fund Test"}
    req = Request(url=url, headers=headers)

    with urlopen(req) as response:
        return response.status == 200


if __name__ == '__main__':
    main()
