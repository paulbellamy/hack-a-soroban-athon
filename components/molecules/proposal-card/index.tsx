import React from 'react'
import ReactMarkdown from 'react-markdown'

export interface IProposalCardProps {
  proposal: any
}

// TODO: This is probably super insecure markdown rendering!
export function ProposalCard(props: IProposalCardProps) {
  const [author, content] = props.proposal

  return (
    <div className="p-4 bg-purple-background-light rounded-lg">
      <p className="text-card-secondary">Submitted by {author.toString()}</p>
      <article className="prose">
        <ReactMarkdown>{Buffer.from(content).toString()}</ReactMarkdown>
      </article>
    </div>
  )
}
