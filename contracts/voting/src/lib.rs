#![no_std]
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, BytesN, Env, IntoVal, RawVal};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_spec.wasm");
}

struct Voting;

#[contractimpl]
impl Voting {
// initialize: set up the contract admins and minimum voting thresholds

// getStatus: {votingEnabled: bool, proposalSubmissionEnabled: bool}

// startSubmissions: Deletes all existing submissions and starts accepting new proposals.

// startVoting: new proposals will not be accepted and voting will start.

// finishVoting: finish the voting process.

// submitProposal: an account submits a proposal that can receive votes. One proposal per account.

// getProposals: gets a list of all proposals available

// getProposals({id}): gets the detail of an available proposal

// verifyEligibility: checks if an account is eligible to voting

// submitVote: submit a vote for an existing proposal
}
