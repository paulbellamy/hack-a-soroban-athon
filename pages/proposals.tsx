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

      <div className="bg-hero-pattern w-full min-h-screen flex flex-col justify-start align-center items-stretch">
        <header className="md:container mx-auto flex flex-row justify-between align-center py-3 px-9 max-w-prose">
          <h1 className="text-md font-semibold uppercase">Sorobounty <span className="bg-purple rounded-full text-tertiary px-3.5">DAO</span></h1>
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
      </div>
    </>
  )
}

export default Proposals
