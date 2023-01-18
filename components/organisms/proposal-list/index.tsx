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

const ProposalList: FunctionComponent = (props: IProposalListProps) => {
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

  const isLoading = (): boolean | undefined => {
    return proposals.loading
  }

  const items = Array.from(proposals.result?.obj()?.map().entries() || []).map(([_, entry]) => [
    entry.key(),
    entry.val().obj()?.bin()
  ]);

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
          />
      )) : <Loading size={64} />}
    </>
  )
}

export { ProposalList }
