import React from 'react'
import type { NextPage } from 'next'
import Head from 'next/head'
import { useContractValue } from '@soroban-react/contracts'
import { useSorobanReact } from '@soroban-react/core'
import * as SorobanClient from 'soroban-client'
import { Loading } from '../components/atoms'
import { ProposalForm, VoterInfo, WalletData } from '../components/molecules'
import { ProposalList } from '../components/organisms'
import { Constants, Phase } from '../shared/constants'
import { accountIdentifier } from '../shared/identifiers'
import { useAccount, useNetwork } from '../wallet'

const Proposals: NextPage = () => {
  const { data: account } = useAccount()
  const { activeChain } = useNetwork()

  const networkPassphrase = activeChain?.networkPassphrase ?? ''

  const sorobanContext = useSorobanReact()
  const phaseQuery = useContractValue({
    contractId: Constants.VotingId,
    method: 'get_status',
    params: [],
    sorobanContext
  })
  const phase = (phaseQuery.result?.u32() ?? Phase.Invalid) as Phase;

  // TODO: This needs to use the current wallet address as the source
  const eligible = useContractValue({
    contractId: Constants.VotingId,
    method: 'eligible',
    params: account ? [accountIdentifier(SorobanClient.StrKey.decodeEd25519PublicKey(account.address))] : [],
    sorobanContext
  })

  const displayPhase = function(phase: Phase) {
    switch (phase) {
    case Phase.Submission:
      return 'Submission phase'
    case Phase.Voting:
      return 'Voting phase'
    case Phase.Finished:
      return 'Finished'
    default:
      return 'Invalid phase'
    }
  };

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
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <link rel="manifest" href="/site.webmanifest" />
        <link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5" />
        <meta name="msapplication-TileColor" content="#da532c" />
        <meta name="theme-color" content="#ffffff" />
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
          {phase < 0 ? (
            <Loading size={64} />
          ) : (
            <>
              <div className="space-y-1">
                <div className="flex justify-start items-center space-x-10">
                  <h3 className="text-3xl font-semibold">Batch #1</h3>
                  {/* TODO: Wire up the phase here */}
                  <span className="text-tertiary font-semibold py-1 px-4 bg-button-information rounded-full">{displayPhase(phase)}</span>
                </div>
                <p>Lorem ipsum...</p>
              </div>
              {phase === Phase.Submission && account?.address && (
                <div className="space-y-1">
                  <h2 className="text-xl">Your Proposals</h2>
                  <ProposalForm
                    isEligible={isEligible}
                    account={account.address}
                    contractId={Constants.VotingId}
                    networkPassphrase={networkPassphrase} />
                </div>
              )}
              {phase === Phase.Voting && account?.address && (
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
