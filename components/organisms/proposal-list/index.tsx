import { useContractValue } from '@soroban-react/contracts'
import { useSorobanReact } from '@soroban-react/core'
import React, { FunctionComponent } from 'react'
import * as SorobanClient from 'soroban-client'
import { Constants } from '../../../shared/constants'
import { Loading } from '../../atoms'
import { ProposalCard } from '../../molecules'
let xdr = SorobanClient.xdr

const ProposalList: FunctionComponent = () => {
  const sorobanContext = useSorobanReact()
  const proposals = useContractValue({ 
    contractId: Constants.VotingId,
    method: 'proposals',
    sorobanContext
  })

  const isLoading = (): boolean | undefined => {
    return proposals.loading
  }

  React.useEffect(() => {
    console.debug('proposals', proposals)
  }, [proposals])

  return isLoading() ? (
    <>
      <h2 className="text-xl">All Proposals (...) | Submission ends 2/10</h2>
      <Loading size={64} />
    </>
  ) : (
    <>
      <h2 className="text-xl">All Proposals (...) | Submission ends 2/10</h2>
      {/*proposals.data?.map((proposal: any, index: number) => {
        <ProposalCard key={index} proposal={proposal} />
      }*/}
    </>
  )
}

export { ProposalList }
