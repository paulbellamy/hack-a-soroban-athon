#![no_std]
use errors::ContractError;
use soroban_sdk::{
    contractimpl, contracttype, map, panic_with_error, symbol, vec, Address, Bytes, Env, Map, Vec,
};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_spec.wasm");
}

const MIN_MARKDOWN_SIZE: u32 = 10;
const MAX_MARKDOWN_SIZE: u32 = 100;

pub struct Voting;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Status {
    Submission = 0,
    Voting = 1,
    Finished = 2,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admins,
    Token,
    Threshold,
    Status,
    Proposals,
    Votes(Address),
    Desc(Address),
}

// fn is_admin(e: &Env, user: &Identifier) -> bool {
//     let key = DataKey::Admins;
//     let admin_vec = e.storage().get_unchecked(&key).unwrap();

//     if admin_vec.contains(user) {
//         return true;
//     }
//     return false;
// }

// fn delete_all_proposals(e: &Env) {
//     e.storage().remove(DataKey::Proposals)
// }

#[contractimpl]
impl Voting {
    // TODO: initialize: set up the contract admins and minimum voting thresholds
    // fn initialize(
    //     e: Env,
    //     admins: Vec<Identifier>, // Who should be admins
    //     token: BytesN<32>,       // What Badge/Token should be used for votes
    //     threshold: u64,          // Voting threshold of token
    // ) {
    //     assert!(!e.storage().has(DataKey::admins), "already initialized");

    //     e.storage().set(DataKey::Admins, admins);
    //     e.storage().set(DataKey::Token, token);
    //     e.storage().set(DataKey::Threshold, token);
    // }

    // TODO: getStatus: Return status enum
    // fn getStatus(e: &Env) -> Status {
    //     e.storage()
    //         .get(DataKey::Status)
    //         .expect("not initialized")
    //         .unwrap()
    // }

    // TODO: setStatus
    // fn setStatus(e: &Env, user: &Identifier, status: Status) {
    //     if !(is_admin(e, user)) {
    //         panic!("user is not an admin")
    //     }

    //     key = DataKey::Status;
    //     cur_status = e.storage().get_unchecked(key).unwrap();

    //     if cur_status == status {
    //         panic!(status is already {cur_status});
    //     }

    //     if cur_status == Status::Voting {
    //         if status == Status::Submission {
    //             panic!("Can't set status to Submission; Currently in Voting status");
    //         }
    //     }

    //     if status == Status::Submission {
    //         delete_all_proposals(e);
    //     }

    //     e.storage().set(key, status)
    // }

    // propose (AKA submitProposal): an account submits a proposal that can receive votes. One proposal per account.
    pub fn propose(env: Env, proposal_markdown: Bytes) {
        if proposal_markdown.len() < MIN_MARKDOWN_SIZE {
            panic_with_error!(&env, ContractError::InputValueTooShort)
        }

        if proposal_markdown.len() > MAX_MARKDOWN_SIZE {
            panic_with_error!(&env, ContractError::InputValueTooLong)
        }

        // Add proposal ID to list of proposals
        let key = DataKey::Proposals;
        let mut proposals: Map<Address, Bytes> = env
            .storage()
            .get(key.clone())
            .unwrap_or(Ok(map![&env])) // If no value set, initialize it.
            .unwrap();
        proposals.set(env.invoker(), proposal_markdown);
        env.storage().set(key.clone(), proposals);
    }

    // getProposals: gets a list of all proposals available
    pub fn proposals(env: Env) -> Map<Address, Bytes> {
        let key = DataKey::Proposals;
        return env
            .storage()
            .get(key.clone())
            .unwrap_or(Ok(map![&env])) // If no value set, initialize it.
            .unwrap();
    }

    // proposal(id) (AKA getProposals({id})): gets the detail of an available proposal
    pub fn proposal(env: Env, address: Address) -> Bytes {
        let proposals = Self::proposals(env.clone());
        if !proposals.contains_key(address.clone()) {
            panic_with_error!(env.clone(), ContractError::ProposalNotFound);
        }
        return proposals
            .get_unchecked(address.clone())
            .unwrap_or(Bytes::new(&env));
    }

    // TODO: verifyEligibility: checks if an account is eligible to voting

    // TODO: submitVote: submit a vote for an existing proposal
}

mod errors;
mod tests;
