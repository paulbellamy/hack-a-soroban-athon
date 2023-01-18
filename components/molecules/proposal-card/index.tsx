import { useSendTransaction } from '@soroban-react/contracts'
import { useSorobanReact } from '@soroban-react/core'
import React from 'react'
import ReactMarkdown from 'react-markdown'
import * as SorobanClient from 'soroban-client'
import { Phase } from '../../../shared/constants'
import { accountIdentifier } from '../../../shared/identifiers'
import { Utils } from '../../../shared/utils'
import { Button } from '../../atoms'
import { useNetwork } from '../../../wallet'

export interface IProposalCardProps {
  account: string
  contractId: string
  networkPassphrase: string
  proposal: any
  phase: Phase
  isEligible: boolean
  votes: number
  winner?: number
}

// TODO: This is probably super insecure markdown rendering!
export function ProposalCard(props: IProposalCardProps) {
  const [author, content] = props.proposal
  const [isSubmitting, setSubmitting] = React.useState(false)
  const { server } = useNetwork()
  const sorobanContext = useSorobanReact()
  const { sendTransaction } = useSendTransaction()

  return (
    <div className="p-4 bg-purple-background-light rounded-lg w-full">
      <p className="text-card-secondary uppercase">Submitted by {author}</p>
      <article className="prose">
        <ReactMarkdown>{Buffer.from(content).toString()}</ReactMarkdown>
      </article>
      {props.phase === Phase.Submission ? (
        <span className="inline-block bg-disabled rounded-full text-tertiary py-1 px-3.5 my-3 font-bold">Voting starts 2/11</span>
      ) : props.phase === Phase.Voting ? (
        <Button
          className="py-1 my-3"
          disabled={!props.isEligible}
          isLoading={isSubmitting}
          onClick={async () => {
            setSubmitting(true)
            if (!server) throw new Error("Not connected to server")

            try {
              const wallet = await server.getAccount(props.account)
              const source = new SorobanClient.Account(wallet.id, wallet.sequence)
              const result = await sendTransaction(
                Utils.contractTransaction(
                  props.networkPassphrase,
                  source,
                  props.contractId,
                  'vote',
                  accountIdentifier(SorobanClient.StrKey.decodeEd25519PublicKey(author)),
                ),
                {sorobanContext}
              )
              console.debug(result)
              window.location.reload()
            } catch (error) {
              alert(error)
            }
            setSubmitting(false)
          }}
          >Add vote</Button>
      ) : (
        <span className={`inline-block rounded-full text-tertiary py-1 px-3.5 my-3 font-normal ${props.winner ? "bg-success" : "bg-button-disabled"}`}>{props.winner && `Winner #${props.winner} |`} {props.votes} Vote{props.votes !== 1 && "s"}</span>
      )}
    </div>
  )
}
