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
          <div className="w-1/2 min-w-fit flex flex-col items-center">
            <h1 className="text-4xl mb-0 font-semibold uppercase">Sorobounty <span className="bg-button-information rounded-full text-tertiary px-3.5">DAO</span></h1>
            <h3 className="text-sm font-normal">v1.20 | Powered by Soroban | Built on Stellar</h3>
            <p className="mt-9">Propose and vote what you'd like to see built on Soroban smart contracts.</p>
            <p className="text-purple mt-9 mb-2">Get Started</p>
            <WalletData />
            <hr className="border-1.5 border-divider rounded-full m-9 w-90" />
            <a href="https://quest.stellar.org/live" className="inline-block border border-button-primary rounded-full bg-card-primary text-button-primary px-4 py-3">
              Learn about Stellar Quest
            </a>
          </div>
        </main>
      </div>
    </>
  )
}

export default Home
