import React from 'react'
import type { NextPage } from 'next'
import Head from 'next/head'
import styles from '../styles/Home.module.css'
import { WalletData } from '../components/molecules'
import { Button } from '../components/atoms'
import { useAccount } from '../wallet'

const Home: NextPage = () => {
  const { data: account } = useAccount()

  React.useEffect(() => {
    if (account) {
      window.location.href = '/proposals';
    }
  }, [account])

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
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <div className="bg-hero-pattern w-full min-h-screen flex flex-col justify-center align-center items-center">
        <main className="w-full p-14">
          <div className="space-y-3 w-1/2 min-w-fit">
            <div>
              <h1 className="text-4xl mb-0"><span className="text-purple">Sorobounty</span> <span className="font-bold">DAO</span></h1>
              <h3 className="text-sm font-normal">v1.20 | Powered by Soroban | Built on Stellar</h3>
            </div>
            <p>Propose and vote what you'd like to see built on Soroban smart contracts.</p>
            <div>
              <p className="text-purple">Get Started</p>
              <WalletData />
            </div>
            <hr className="border-1 border-divider" />
            <a href="https://quest.stellar.org/live" className="inline-block border border-button-primary rounded-full bg-button-secondary text-button-primary px-4 py-3">
              Learn about Stellar Quest
            </a>
          </div>
        </main>
      </div>
    </>
  )
}

export default Home
