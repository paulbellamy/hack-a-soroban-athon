use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // You need to be a user to invoke this function
    CrossContractCallProhibited = 1,
    // Input Value too short, provide at least 10 characters (bytes)
    InputValueTooShort = 2,
    // Input Value too long, provide less than 101 (bytes)
    InputValueTooLong = 3,
    // The desired proposal could not be found
    ProposalNotFound = 4,
    // Reached the max vote count for a user
    MaxUserVoteCountReached = 5,
    // The user is not eligible to vote
    UserNotEligible = 6,
    // The contract is not receiving submissions
    NotAcceptingSubmissions = 7,
    // The contract is not receiving votes
    NotAcceptingVotes = 8,
}
