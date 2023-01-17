import React from 'react'
import ReactMarkdown from 'react-markdown'
import * as SorobanClient from 'soroban-client'

export interface IProposalCardProps {
  proposal: any
}

// TODO: This is probably super insecure markdown rendering!
export function ProposalCard(props: IProposalCardProps) {
  const [author, content] = props.proposal

  // TODO: This only handles account keys, and badly.
  const authorDisplayName = SorobanClient.StrKey.encodeEd25519PublicKey(author.obj().vec()[1].obj().accountId().ed25519());

  return (
    <div className="p-4 bg-purple-background-light rounded-lg w-full">
      <p className="text-card-secondary uppercase">Submitted by {authorDisplayName}</p>
      <article className="prose">
        <ReactMarkdown>{Buffer.from(content).toString()}</ReactMarkdown>
      </article>
    </div>
  )
}
