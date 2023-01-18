import React from 'react'
import type { NextPage } from 'next'
import Head from 'next/head'
import styles from '../styles/Home.module.css'
import { WalletData } from '../components/molecules'
import { Button } from '../components/atoms'

const Home: NextPage = () => {
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

      <main className="md:container p-9">
        <div className="space-y-1 w-1/2 min-w-fit">
          <h1 className="text-3xl"><span className="text-purple">Sorobounty</span> <span className="font-bold">DAO</span></h1>
          <h3 className="text-sm">v1.20 | Powered by Soroban | Built on Stellar</h3>
          <p>Propose and vote what you'd like to see built on Soroban smart contracts.</p>
          <p className="text-purple">Get Started</p>
          <WalletData />
          <hr className="border-1 border-divider" />
          <a href="https://quest.stellar.org/live" className="inline-block border border-button-primary rounded-full bg-button-secondary text-button-primary px-4 py-3">
            Learn about Stellar Quest
          </a>
        </div>
      </main>
    </>
  )
}

export default Home
