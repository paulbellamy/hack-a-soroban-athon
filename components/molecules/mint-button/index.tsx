import BigNumber from 'bignumber.js'
import { useContractValue, useSendTransaction } from '@soroban-react/contracts'
import { useSorobanReact } from '@soroban-react/core'
import React from 'react'
import ReactMarkdown from 'react-markdown'
import * as SorobanClient from 'soroban-client'
import { Constants } from '../../../shared/constants'
import { accountIdentifier } from '../../../shared/identifiers'
import { Utils } from '../../../shared/utils'
import { Button, Loading } from '../../atoms'
import { useNetwork } from '../../../wallet'
import * as convert from '../../../convert'

export interface IMintButtonProps {
  account: string
}


// MintButton mints 100.0000000 tokens to the user's wallet for testing
export function MintButton(props: IMintButtonProps) {
  const [isSubmitting, setSubmitting] = React.useState(false)
  const { activeChain, server } = useNetwork()
  const networkPassphrase = activeChain?.networkPassphrase ?? ''
  const sorobanContext = useSorobanReact()

  const { sendTransaction } = useSendTransaction()
  const amount = BigNumber(1).shiftedBy(-7).toString()

  const symbolValue = useContractValue({
    contractId: Constants.TokenId,
    method: 'symbol',
    params: [],
    sorobanContext
  })

  const balanceValue = useContractValue({
    contractId: Constants.TokenId,
    method: 'balance',
    params: [accountIdentifier(SorobanClient.StrKey.decodeEd25519PublicKey(props.account))],
    sorobanContext
  })

  const symbol = symbolValue.result?.obj()?.bin().toString().replace(/\0/g, '')
  const balance = convert.scvalToBigNumber(balanceValue.result)
  if (balanceValue.loading || !symbol) {
    return <Loading size={16} />
  }

  return (
    <>
      <Button
        invert
        onClick={async () => {
          setSubmitting(true)

          if (!server) throw new Error("Not connected to server")

          let { sequence } = await server.getAccount(Constants.TokenAdmin)
          let adminSource = new SorobanClient.Account(Constants.TokenAdmin, sequence)

          let wallet = await server.getAccount(props.account)
          let walletSource = new SorobanClient.Account(wallet.id, wallet.sequence)

          //
          // 1. Establish a trustline to the admin (if necessary)
          // 2. The admin sends us money (mint)
          //
          // We have to do this in two separate transactions because one
          // requires approval from Freighter while the other can be done with
          // the stored token issuer's secret key.
          //
          // FIXME: The `getAccount()` RPC endpoint doesn't return `balances`,
          //        so we never know whether or not the user needs a trustline
          //        to receive the minted asset.
          //
          // Today, we establish the trustline unconditionally.
          //
          try {
            console.log("sorobanContext: ", sorobanContext)
            const trustlineResult = await sendTransaction(
              new SorobanClient.TransactionBuilder(walletSource, {
                networkPassphrase,
                fee: "1000", // arbitrary
              })
              .setTimeout(60)
              .addOperation(
                SorobanClient.Operation.changeTrust({
                  asset: new SorobanClient.Asset(symbol, Constants.TokenAdmin),
                })
              )
              .build(), {
                timeout: 60 * 1000, // should be enough time to approve the tx
                skipAddingFootprint: true, // classic = no footprint
                // omit `secretKey` to have Freighter prompt for signing
                // hence, we need to explicit the sorobanContext
                sorobanContext
              },
            )
            console.debug(trustlineResult)
          } catch (err) {
            console.error(err)
          }

          try {
            const paymentResult = await sendTransaction(
              new SorobanClient.TransactionBuilder(adminSource, {
                networkPassphrase,
                fee: "1000",
              })
              .setTimeout(10)
              .addOperation(
                SorobanClient.Operation.payment({
                  destination: wallet.id,
                  asset: new SorobanClient.Asset(symbol, Constants.TokenAdmin),
                  amount: amount.toString(),
                })
              )
              .build(), {
                timeout: 10 * 1000,
                skipAddingFootprint: true,
                secretKey: Constants.TokenAdminSecretKey,
                sorobanContext
              }
            )
            console.debug(paymentResult)
          } catch (err) {
            console.error(err)
          }
          //
          // TODO: Show some user feedback while we are awaiting, and then based
          // on the result
          //
          window.location.reload()
        }}
        disabled={isSubmitting}
        isLoading={isSubmitting}
      >Claim {symbol}</Button>
      <span>{balance.toString()} {symbol}</span>
    </>
  )
}
