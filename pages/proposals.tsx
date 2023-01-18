import React from 'react'
import type { NextPage } from 'next'
import Head from 'next/head'
import { ProposalForm, WalletData } from '../components/molecules'
import { ProposalList } from '../components/organisms'
import { Constants } from '../shared/constants'
import { useAccount, useNetwork } from '../wallet'

const Proposals: NextPage = () => {
  const { data: account } = useAccount()
  const { activeChain } = useNetwork()

  const networkPassphrase = activeChain?.networkPassphrase ?? ''

  return (
    <>
      <Head>
        <title>
          Sorobounty DAO - Propose and vote what you'd like to see built on
          Soroban smart contracts.
        </title>
        <meta
          name="description"
          content="Propose and vote what you'd like to see built on Soroban smart contracts"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <header className="flex flex-row justify-between align-center px-9">
        <h1 className="text-md"><span className="text-purple">Sorobounty</span> <span className="font-bold">DAO</span></h1>
        <WalletData />
      </header>
      <main className="md:container p-9 mx-auto max-w-prose align-center space-y-6">
        <div className="space-y-1">
          <h2 className="text-xl">Propose and vote what you'd like to see built on Soroban</h2>
          <p>Lorem ipsum...</p>
        </div>
        {account?.address && (
          <div className="space-y-1">
            <h2 className="text-xl">Your Proposals</h2>
            <ProposalForm
              account={account.address}
              contractId={Constants.VotingId}
              networkPassphrase={networkPassphrase} />
          </div>
        )}
        <div className="space-y-1 w-full">
          <ProposalList />
        </div>
      </main>
    </>
  )
}

export default Proposals
