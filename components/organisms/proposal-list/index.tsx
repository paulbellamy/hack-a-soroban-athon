import { useContractValue } from '@soroban-react/contracts'
import { useSorobanReact } from '@soroban-react/core'
import React, { FunctionComponent } from 'react'
import * as SorobanClient from 'soroban-client'
import { Constants, Phase } from '../../../shared/constants'
import { useAccount, useNetwork } from '../../../wallet'
import { Loading } from '../../atoms'
import { ProposalCard } from '../../molecules'
let xdr = SorobanClient.xdr

export interface IProposalListProps {
  phase: Phase
  isEligible: boolean
}

function ProposalList(props: IProposalListProps) {
  const { data: account } = useAccount()
  const { activeChain } = useNetwork()

  const networkPassphrase = activeChain?.networkPassphrase ?? ''

  const sorobanContext = useSorobanReact()
  const proposals = useContractValue({
    contractId: Constants.VotingId,
    method: 'proposals',
    params: [],
    sorobanContext
  })

  const results = useContractValue({
    contractId: Constants.VotingId,
    method: 'results',
    params: [],
    sorobanContext
  })

  const isLoading = (): boolean | undefined => {
    return proposals.loading || (props.phase === Phase.Finished && results.loading)
  }

  const items = Array.from(proposals.result?.obj()?.map().entries() || []).map(([_, entry]) => [
    parseAccountId(entry.key()),
    entry.val().obj()?.bin(),
  ])

  const votes = Object.fromEntries(Array.from(results.result?.obj()?.map().entries() || []).map(([_, entry]) => [
    parseAccountId(entry.key()),
    entry.val().u32(),
  ]))

  // If we are in the finished phase, we need to sort the results by their number of votes
  if (props.phase === Phase.Finished) {
    items.sort((a, b) => {
      const aVotes = (a && a[0] && votes[a[0].toString()]) || 0
      const bVotes = (b && b[0] && votes[b[0].toString()]) || 0
      return bVotes - aVotes
    })
  }

  return isLoading() ? (
    <>
      <h2 className="text-xl">All Proposals (...) | Submission ends 2/10</h2>
      <Loading size={64} />
    </>
  ) : (
    <>
      <h2 className="text-xl">All Proposals ({items.length}) | Submission ends 2/10</h2>
      {account?.address ? items.map((proposal: any, index: number) => (
        <ProposalCard
          key={index}
          account={account.address}
          contractId={Constants.VotingId}
          networkPassphrase={networkPassphrase}
          proposal={proposal}
          phase={props.phase}
          isEligible={props.isEligible}
          votes={votes[proposal[0]] ?? 0}
          winner={index < 5 ? index + 1 : undefined}
          />
      )) : <Loading size={64} />}
    </>
  )
}

export { ProposalList }

// TODO: This only handles account keys, and badly.
function parseAccountId(scval: SorobanClient.xdr.ScVal): string {
    return SorobanClient.StrKey.encodeEd25519PublicKey(scval.obj()!.vec()[1].obj()!.accountId().ed25519())
}
