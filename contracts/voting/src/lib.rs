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

// const ELIGIBLE_USERS: &'static [&'static str] = &[
//     "GBWAN65QEOJX3XKOCYRHFB3VG5EPUJIPN5T47YVTATT2WRK23UA7WLEX", 
//     "GDLV5FAXOUL4DMLHLQOYWHU4V4PRG7CQACYYI7LY2VFMLAWAD7ZT3VL2",
//     "GCVLLUMASL5ZOFZXVJ22KWO5HWFT2IH2Q3HUZFP5AV2K5IRPBYGCBRWJ"
//     ];

pub struct VotingContract;

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
impl VotingContract {
    // TODO: initialize: set up the contract admins and minimum voting thresholds
    // pub fn initialize(
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
    // pub fn getStatus(e: &Env) -> Status {
    //     e.storage()
    //         .get(DataKey::Status)
    //         .expect("not initialized")
    //         .unwrap()
    // }

    // TODO: setStatus
    // pub fn setStatus(e: &Env, user: &Identifier, status: Status) {
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
        //  Only an invoker of the `AccountId` type (i.e. an actual user) can invoke this function.
        match env.invoker() {
            Address::Account(account_id) => account_id,
            Address::Contract(_) => {
                panic_with_error!(&env, ContractError::CrossContractCallProhibited)
            }
        };

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

    // eligible(id) (AKA verifyEligibility): checks if an account is eligible to voting
    pub fn eligible(env: Env) -> bool {
        let key = match env.invoker() {
                Address::Account(account_id) => account_id,
                Address::Contract(_) => {
                    panic_with_error!(&env, ContractError::CrossContractCallProhibited)
                }
            };

        // if ELIGIBLE_USERS.contains(key) {
        //     return true;
        // }
        //
        // return false;

        return true;
    }


    // TODO: verifyEligibility: checks if an account is eligible to voting

    // TODO: submitVote: submit a vote for an existing proposal

    // TODO: getResults: get the results of the votes. Only available after the voting period is over?
}

mod errors;
mod tests;
