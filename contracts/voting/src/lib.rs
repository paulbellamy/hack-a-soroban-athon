#![no_std]
use error::ContractError;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, panic_with_error, log, BytesN, Env, IntoVal, RawVal, Symbol, Vec, vec, symbol, Address, AccountId, Bytes, bytes};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_spec.wasm");
}

pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    // Hello World function
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
    
    /// The Submit Proposal `sbt_prop()` function takes a `value` parameter, accepting a Bytes object
    /// for it. This argument can be supplied an array of u8 values, an integer,
    /// or a hex-encoded string.
    pub fn sbmt_prop(env: Env, value: Bytes) -> Result<(), ContractError> {
        // We are using the `panic!` macro to ensure that this function cannot
        // be cross-called from another contract. Only an invoker of the
        // `AccountId` type, which is the identifier of a Stellar account
        // (ed25519 public key), can invoke this function.
        let key = match env.invoker() {
            Address::Account(account_id) => account_id,
            Address::Contract(_) => {
                panic_with_error!(&env, ContractError::CrossContractCallProhibited)
            }
        };

        // We are ensuring the provided Bytes value length is at least 11 since
        // we want users to perform the String to Bytes conversion on their own,
        // without passing simple values like Bytes(7). We also want to
        // highlight some differences between Bytes and symbols (which must be
        // 10 or fewer characters).
        if value.len() <= 10 {
            panic_with_error!(&env, ContractError::InputValueTooShort)
        }

        // We then use `env.storage().set()` to store the value that was passed,
        // associating it with the contract invoker's AccountId.
        env.storage().set(key, value);

        Ok(()) // return ok if function call succeeded
    }


    /// The Get Proposal `get_prop()` function takes an `owner` parameter, accepting an AccountId
    /// object for it. We then use `env.storage().get()` to retrieve the value
    /// which has been associated with the supplied AccountId. If there is no
    /// data associated, return Bytes of length 0.
    pub fn get_prop(env: Env, owner: AccountId) -> Bytes {
        // Hmm. Interesting. This function doesn't enforce an `AccountId` type
        // of invoker. I guess this function *could* be invoked by another
        // contract. I wonder if that will be useful at some point?
        env.storage()
            .get(owner)
            .unwrap_or_else(|| Ok(bytes!(&env))) // This uses `unwrap_or_else` and closure which only evaluates Bytes(0) when necessary.
            .unwrap()
    }




    
// verifyEligibility: checks if an account is eligible to voting

// getProposals: gets a list of all proposals available

// getProposals({id}): gets the detail of an available proposal

// submitVote: submit a vote for an existing proposal

// initialize: set up the contract admins and minimum voting thresholds

// getStatus: {votingEnabled: bool, proposalSubmissionEnabled: bool}

// startSubmissions: Deletes all existing submissions and starts accepting new proposals.

// startVoting: new proposals will not be accepted and voting will start.

// finishVoting: finish the voting process.

// submitProposal: an account submits a proposal that can receive votes. One proposal per account.
}

mod error;
mod test;