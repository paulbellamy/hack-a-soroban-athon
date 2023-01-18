import React from 'react'
import type { NextPage } from 'next'
import Head from 'next/head'
import { useContractValue } from '@soroban-react/contracts'
import { useSorobanReact } from '@soroban-react/core'
import { Loading } from '../components/atoms'
import { ProposalForm, VoterInfo, WalletData } from '../components/molecules'
import { ProposalList } from '../components/organisms'
import { Constants, Phase } from '../shared/constants'
import { useAccount, useNetwork } from '../wallet'

const Proposals: NextPage = () => {
  const { data: account } = useAccount()
  const { activeChain } = useNetwork()

  const networkPassphrase = activeChain?.networkPassphrase ?? ''

  const sorobanContext = useSorobanReact()
  const phaseQuery = useContractValue({
    contractId: Constants.VotingId,
    method: 'getStatus',
    params: [],
    sorobanContext
  })
  const phase = phaseQuery.result?.sym()?.toString() as Phase | undefined;

  // TODO: This needs to use the current wallet address as the source
  const eligible = useContractValue({
    contractId: Constants.VotingId,
    method: 'eligible',
    params: [],
    sorobanContext
  })

  const displayPhase = phase && {
    'submission': 'Submission phase',
    'voting': 'Voting phase',
    'finished': 'Finished'
  }[phase];

  const isEligible = eligible.result?.ic().name == 'scsTrue'

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
          <h1 className="text-md font-semibold uppercase">Sorobounty <span className="bg-button-information rounded-full text-tertiary px-3.5">DAO</span></h1>
          <WalletData />
        </header>
        <main className="md:container p-9 mx-auto max-w-prose align-center space-y-6">
          <div className="space-y-1">
            <h2 className="text-4xl">Propose and vote what you'd like to see built on Soroban</h2>
            <p>Lorem ipsum...</p>
          </div>
          {!phase ? (
            <Loading size={64} />
          ) : (
            <>
              <div className="space-y-1">
                <div className="flex justify-start items-center space-x-10">
                  <h3 className="text-3xl font-semibold">Batch #1</h3>
                  {/* TODO: Wire up the phase here */}
                  <span className="text-tertiary font-semibold py-1 px-4 bg-button-information rounded-full">{displayPhase}</span>
                </div>
                <p>Lorem ipsum...</p>
              </div>
              {phase == 'submission' && account?.address && (
                <div className="space-y-1">
                  <h2 className="text-xl">Your Proposals</h2>
                  <ProposalForm
                    account={account.address}
                    contractId={Constants.VotingId}
                    networkPassphrase={networkPassphrase} />
                </div>
              )}
              {phase == 'voting' && account?.address && (
                <div className="space-y-1">
                  <h2 className="text-xl">Your Vote</h2>
                  {eligible.loading
                    ? <Loading size={64} />
                    : <VoterInfo isEligible={isEligible} />}
                </div>
              )}
              <div className="space-y-1 w-full">
                <ProposalList phase={phase} isEligible={isEligible} />
              </div>
            </>
          )}
        </main>
      </div>
    </>
  )
}

export default Proposals