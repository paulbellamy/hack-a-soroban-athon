import React from 'react'

export interface IVoterInfoProps {
  isEligible: boolean
}

export function VoterInfo(props: IVoterInfoProps) {
  return (
    <div className="flex flex-col justify-center align-center items-center p-3 w-full space-y-3">
      <p className="font-semibold">You're {props.isEligible ? "eligible" : "not eligible"} to vote.</p>
      <p>Vote before 2/20</p>
    </div>
  )
}
