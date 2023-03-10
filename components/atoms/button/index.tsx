import React, { ReactNode } from 'react'
import { Loading } from '../loading'

export interface ButtonProps {
  className?: string
  children: ReactNode
  onClick: () => void
  disabled?: boolean
  isLoading?: boolean
  invert?: boolean
}

export function Button({ className, children, onClick, disabled, isLoading, invert}: ButtonProps) {
  className = [
    "inline-block",
    "border",
    "rounded-full",
    "px-4",
    "py-1",
    "leading-loose",
    "font-bold",
    "cursor-pointer",
    "border-button-primary",
    (disabled
      ? "bg-disabled text-tertiary cursor-not-allowed border-tertiary"
      : invert
      ? "bg-card-primary text-button-primary"
      : "bg-button-primary text-button-secondary"
      ),
    className,
  ].filter(x => !!x).join(" ")
  return (
    <button className={className} onClick={onClick} disabled={disabled}>
      {isLoading ? <Loading size={18} /> : children}
    </button>
  )
}
