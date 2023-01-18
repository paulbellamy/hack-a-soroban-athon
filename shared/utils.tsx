import BigNumber from 'bignumber.js'
import humanizeDuration from 'humanize-duration'
import moment from 'moment'
import * as SorobanClient from 'soroban-client'

const formatAmount = (value: BigNumber, decimals = 7): string => {
  return value.shiftedBy(decimals * -1).toNumber().toLocaleString()
}

const getRemainingTime = (date?: Date): string => {
  if (!date) {
    return 'Undefined'
  }
  const diff = moment(date).diff(Date.now())

  if (isExpired(date)) {
    return 'Expired'
  }

  return (
    humanizeDuration(diff, {
      round: true,
      conjunction: ' and ',
      largest: 1,
    }) + ' left'
  )
}

const isExpired = (date?: Date): boolean => {
  return moment(date).diff(Date.now()) <= 0
}

const percentage = (
  value: BigNumber,
  divider: BigNumber,
  decimals = 7
): number => {
  return (
    (value.shiftedBy(decimals * -1).toNumber() /
      divider.shiftedBy(decimals * -1).toNumber()) *
    100
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

const Utils = {
  formatAmount,
  getRemainingTime,
  isExpired,
  percentage,
  contractTransaction,
}

export { Utils }
