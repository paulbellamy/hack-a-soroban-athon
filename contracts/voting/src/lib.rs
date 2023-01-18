#![no_std]
use errors::ContractError;
use soroban_sdk::{
    contractimpl, contracttype, map, panic_with_error, Address, Bytes, Env, Map, AccountId, IntoVal, RawVal, BytesN, TryFromVal, ConversionError
};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_spec.wasm");
}

const MIN_MARKDOWN_SIZE: u32 = 10;
const MAX_MARKDOWN_SIZE: u32 = 100;

const MAX_USER_VOTE_COUNT: u32 = 1;

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
    Invalid = 3,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Threshold,
    Status,
    Proposals,
    PropVotes(Address), // Vote count for each Proposal Address
    UserVotes(AccountId) // Vote count for each User Account (max 1 vote per user for the MVP)
}

//impl IntoVal<Env, RawVal> for Status {
//    fn into_val(self, env: &Env) -> RawVal {
//        (self as u32).into_val(env)
//    }
//}
//
//impl TryFromVal<Env, RawVal> for Status {
//    type Error = ConversionError;
//
//    fn try_from_val(_env: &Env, v: RawVal) -> Result<Self, Self::Error> {
//        let value = v.get_payload();
//
//        if value == Status::Submission as u64 {
//            return Ok(Status::Submission);
//        }
//
//        if value == Status::Voting as u64 {
//            return Ok(Status::Voting);
//        }
//
//        if value == Status::Finished as u64 {
//            return Ok(Status::Finished);
//        }
//
//        Ok(Status::Invalid)
//    }
//}

fn is_admin(e: &Env, user: AccountId) -> bool {
    let admin_user: AccountId = e.storage().get(DataKey::Admin).expect("not initialized").unwrap();

    if admin_user == user {
        return true;
    }
    false
}

fn delete_all_proposals(e: &Env) {
    e.storage().remove(DataKey::Proposals)
}

#[contractimpl]
impl VotingContract {
    // initialize: set up the contract admins and minimum voting thresholds
    pub fn initialize(
        e: Env,
        admin: AccountId, // Who should be the admin
        token: BytesN<32>,       // What Badge/Token should be used for votes
        threshold: u64,          // Voting threshold of token
    ) {
        e.storage().set(DataKey::Admin, admin);
        e.storage().set(DataKey::Token, token);
        e.storage().set(DataKey::Threshold, threshold);
        e.storage().set(DataKey::Status, 0 as u64);
    }

    // getStatus: Return status enum
    // NOTE: Status is currently hardcoded as u64 as a hack around enum issues
    pub fn get_status(e: Env) -> u64 {
        e.storage()
            .get(DataKey::Status)
            .expect("not initialized")
            .unwrap()
    }

    // setStatus
    pub fn set_status(e: Env, user: AccountId, status: u64) {
        if !(is_admin(&e, user)) {
            panic!("user is not an admin")
        }
    
        let cur_status: u64 = e.storage().get_unchecked(DataKey::Status).unwrap();
    
        if cur_status == status {
            return
        }
    
        if cur_status == 1 {
            if status == 0 {
                panic!("Can't set status to Submission; Currently in Voting status");
            }
        }
    
        if status == 0 {
            delete_all_proposals(&e);
        }
    
        e.storage().set(DataKey::Status, status)
    }

    // get_admin
    pub fn get_admin(e: Env) -> AccountId {
        e.storage()
            .get(DataKey::Admin)
            .expect("not initialized")
            .unwrap()
    }

    // get_token
    pub fn get_token(e: Env) -> BytesN<32> {
        e.storage()
            .get(DataKey::Token)
            .expect("not initialized")
            .unwrap()
    }

    // get_thresh
    pub fn get_thresh(e: Env) -> u64 {
        e.storage()
            .get(DataKey::Threshold)
            .expect("not initialized")
            .unwrap()
    }

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

    // vote(id) (AKA submitVote({id})): submit a vote for an existing proposal
    pub fn vote(env: Env, proposal_address: Address) {
        //  Only an invoker of the `AccountId` type (i.e. an actual user) can invoke this function.
        let user_id: AccountId = match env.invoker() {
            Address::Account(account_id) => account_id,
            Address::Contract(_) => {
                panic_with_error!(&env, ContractError::CrossContractCallProhibited)
            }
        };

        let user_votes_count: u32 = env
            .storage()
            .get(&user_id)
            .unwrap_or(Ok(0)) // If no value set, initialize it.
            .unwrap();
        
        if user_votes_count >= MAX_USER_VOTE_COUNT {
            panic_with_error!(&env, ContractError::MaxUserVoteCountReached)
        }
        
        let proposal_votes_count: u32 = env
            .storage()
            .get(&proposal_address)
            .unwrap_or(Ok(0)) // If no value set, initialize it.
            .unwrap();

        env.storage().set(&proposal_address, proposal_votes_count + 1);
        env.storage().set(&user_id, user_votes_count + 1);
    }

    // TODO: getResults: get the results of the votes. Only available after the voting period is over?
}

mod errors;
mod tests;
