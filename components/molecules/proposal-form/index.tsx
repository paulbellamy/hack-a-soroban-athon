import { useContractValue, useSendTransaction } from '@soroban-react/contracts'
import { useSorobanReact } from '@soroban-react/core'
import React from 'react'
import * as SorobanClient from 'soroban-client'
import { Loading, TextArea, Button } from '../../atoms'
import { useNetwork } from '../../../wallet'
import { Constants } from '../../../shared/constants'
import { accountIdentifier } from '../../../shared/identifiers'
let xdr = SorobanClient.xdr

export interface IProposalFormProps {
  account: string
  contractId: string
  networkPassphrase: string
}

interface Proposal {
  content: string
}

const placeholder = `# Proposal Title

All about your proposal...`;

export function ProposalForm(props: IProposalFormProps) {
  const [isSubmitting, setSubmitting] = React.useState(false)
  const { server } = useNetwork()
  const sorobanContext = useSorobanReact()
  const { sendTransaction } = useSendTransaction()
  const [content, setContent] = React.useState<string|null>(null)


  const pubkey = React.useMemo(
    () => SorobanClient.StrKey.decodeEd25519PublicKey(props.account),
    [props.account]
  );
  const proposal = useContractValue({
    contractId: Constants.VotingId,
    method: 'proposal',
    params: [accountIdentifier(pubkey)],
    sorobanContext
  })

  React.useEffect(() => {
    if (proposal.result) {
      setContent(proposal.result.obj()?.bin().toString() || null)
    }
  }, [proposal.result])


  // Check if we have a proposal already

  return (
    <div className="flex flex-col justify-center align-center items-center p-3 w-full space-y-3">
      {proposal.loading ? (
        <Loading size={64} />
      ) : content !== null ? (
        <>
          <TextArea
            name="proposal"
            placeholder={placeholder}
            input={content}
            setInput={setContent}
            />
          <Button
            onClick={async () => {
              setSubmitting(true)
              if (!server) throw new Error("Not connected to server")

              try {
                const wallet = await server.getAccount(props.account)
                const source = new SorobanClient.Account(wallet.id, wallet.sequence)
                const result = await sendTransaction(
                  contractTransaction(
                    props.networkPassphrase,
                    source,
                    props.contractId,
                    'propose',
                    xdr.ScVal.scvObject(
                      xdr.ScObject.scoBytes(Buffer.from(content))
                    )
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
            >{proposal.result ? "Update" : "Submit"} proposal</Button>
        </>
      ) : (
        <>
          <p>You have no submissions yet.</p>
          <p className="font-semibold">Submit your proposal before 2/10</p>
          <Button
            onClick={() => {
              setContent("")
            }}
            >Create a proposal</Button>
        </>
      )}
    </div>
  )
}

// Small helper to build a contract invokation transaction
function contractTransaction(
  networkPassphrase: string,
  source: SorobanClient.Account,
  contractId: string,
  method: string,
  ...params: SorobanClient.xdr.ScVal[]
): SorobanClient.Transaction {
  const contract = new SorobanClient.Contract(contractId)
  return new SorobanClient.TransactionBuilder(source, {
      // TODO: Figure out the fee
      fee: '100',
      networkPassphrase,
    })
    .addOperation(contract.call(method, ...params))
    .setTimeout(SorobanClient.TimeoutInfinite)
    .build()
}

