#![no_std]
use soroban_sdk::contractimpl;

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_spec.wasm");
}

struct Voting;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Status {
    Submission = 0,
    Voting = 1,
    Finished = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DataKey {
    Admins,
    Token,
    Threshold,
    Status,
    Proposals,
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
    // initialize: set up the contract admins and minimum voting thresholds
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

    // getStatus: Return status enum
    // fn getStatus(e: &Env) -> Status {
    //     e.storage()
    //         .get(DataKey::Status)
    //         .expect("not initialized")
    //         .unwrap()
    // }

    // setStatus
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

    // submitProposal: an account submits a proposal that can receive votes. One proposal per account.

    // getProposals: gets a list of all proposals available

    // getProposals({id}): gets the detail of an available proposal

    // verifyEligibility: checks if an account is eligible to voting

    // submitVote: submit a vote for an existing proposal
}
