import React from 'react'
import { useAccount, useIsMounted, useNetwork } from '../../../wallet'
import { ConnectButton } from '../../atoms'
import Image from 'next/image'

// TODO: Eliminate flash of unconnected content on loading
export function WalletData() {
  const mounted = useIsMounted()

  const { data: account } = useAccount()

  const { activeChain: chain, chains } = useNetwork()

  const unsupportedChain = chain?.unsupported

  return (
    <>
      {mounted && account ? (
        <div className="flex flex-row justify-center items-center space-x-2">
          {chain && (chains.length > 1 || unsupportedChain) && (
            <Card>
              {chain.iconUrl && (
                <Image
                  alt={chain.name ?? 'Chain icon'}
                  style={{
                    background: chain.iconBackground,
                  }}
                  height="24"
                  src={chain.iconUrl}
                  width="24"
                />
              )}
              {chain.name ?? chain.id}
            </Card>
          )}
          <Card>{account.displayName}</Card>
        </div>
      ) : (
        <ConnectButton label="Connect Wallet" />
      )}
    </>
  )
}

function Card({children}: {children: React.ReactNode}) {
  return (
    <div className="border rounded-full px-4 py-1 font-bold border-button-primary bg-card-primary text-button-primary flex flex-row align-center justify-center leading-loose">
      {children}
    </div>
  )
}
