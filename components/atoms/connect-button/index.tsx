import React from 'react'
import { useSorobanReact } from '@soroban-react/core'
import { Button } from "../"
import styles from './style.module.css'

export interface ConnectButtonProps {
  label: string
  isHigher?: boolean
}

export function ConnectButton({ label }: ConnectButtonProps) {
  const { connect } = useSorobanReact()
  const openConnectModal = async () => {
    await connect()
  }

  return (
    <Button
      onClick={openConnectModal}
    >
      {label}
    </Button>
  )
}
