import React from 'react'

export interface IProposalCardProps {
  proposal: any
}

export function ProposalCard(props: IProposalCardProps) {
  const [author, content] = props.proposal

  return (
    <div className="p-4 bg-purple-background-light rounded-lg">
      <p className="text-card-secondary">Submitted by {author.toString()}</p>
      {Buffer.from(content).toString()}
    </div>
  )
}
