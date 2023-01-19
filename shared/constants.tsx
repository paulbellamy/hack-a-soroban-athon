// These were randomly generated from https://laboratory.stellar.org/#account-creator
const TokenAdmin = 'GBHCFFI5ZCRNHW6RZVR5WG7ERSP7NOOVKF36QBB4PVESDCMZQL6OQYEM'
const TokenAdminSecretKey = 'SC4OVVO2RKSUXWAPRMOQSHDZL2Q5COBGOFZSF5MWVTYXC2LHVFFSAVEJ'

// Contract IDs, set up by ./initialize.sh
const VotingId = process.env.VOTING_ID ?? ''
const TokenId = process.env.TOKEN_ID ?? ''

const Constants = {
  VotingId,
  TokenAdmin,
  TokenAdminSecretKey,
  TokenId,
}

enum Phase {
  Submission = 0,
  Voting = 1,
  Finished = 2,
  Invalid = 3,
}

export { Constants, Phase }
