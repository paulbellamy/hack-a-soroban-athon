import React, { Dispatch, SetStateAction } from 'react'

export interface TextAreaProps {
  name: string
  placeholder?: string
  input: string
  setInput: Dispatch<SetStateAction<string>>
}

export function TextArea({ name, placeholder, input, setInput }: TextAreaProps) {
  const handleChange = (event: {
    target: { name: string; value: string }
  }): void => {
    setInput(event.target.value)
  }

  return (
    <textarea
      className="bg-card-background w-full p-3 rounded-lg min-h-[200px]"
      name={name}
      placeholder={placeholder}
      onChange={handleChange}
      value={input}
      autoComplete="off"
    />
  )
}
