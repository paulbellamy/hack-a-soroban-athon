#![no_std]

use errors::ContractError;
use soroban_sdk::{
    contractimpl, contracttype, map, panic_with_error, Address, Bytes, BytesN, Env, Map,
};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_spec.wasm");
}

const MIN_MARKDOWN_SIZE: u32 = 10;
const MAX_MARKDOWN_SIZE: u32 = 2000;

const MAX_USER_VOTE_COUNT: u32 = 1;

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
    PropsVotes,
    UsersVotes,
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
//        if value == Status::Submission as u32 {
//            return Ok(Status::Submission);
//        }
//
//        if value == Status::Voting as u32 {
//            return Ok(Status::Voting);
//        }
//
//        if value == Status::Finished as u32 {
//            return Ok(Status::Finished);
//        }
//
//        Ok(Status::Invalid)
//    }
//}

fn is_admin(env: &Env, user: Address) -> bool {
    let admin_user: Address = env
        .storage()
        .get(DataKey::Admin)
        .expect("not initialized")
        .unwrap();

    if admin_user == user {
        return true;
    }
    false
}

fn delete_all_proposals(env: &Env) {
    env.storage().remove(DataKey::Proposals)
}

fn get_balance(env: &Env, user: Address) -> i128 {
    let badges_token_id: BytesN<32> = env
        .storage()
        .get(DataKey::Token)
        .expect("not initialized")
        .unwrap();
    let token_client = token::Client::new(&env, badges_token_id);
    token_client.balance(&user.into())
}

fn get_threshold(env: &Env) -> u32 {
    env.storage()
        .get(DataKey::Threshold)
        .expect("not initialized")
        .unwrap()
}

fn is_eligible(env: &Env, user: Address) -> bool {
    let _key = match &user {
        Address::Account(account_id) => account_id,
        Address::Contract(_) => {
            panic_with_error!(&env, ContractError::CrossContractCallProhibited)
        }
    };

    // Check their token balance
    let threshold = get_threshold(&env);
    let balance = get_balance(&env, user);
    if balance < threshold as i128 {
        return false;
    }

    return true;
}

#[contractimpl]
impl VotingContract {
    // initialize: set up the contract admins and minimum voting thresholds
    pub fn initialize(
        env: Env,
        admin: Address,    // Who should be the admin
        token: BytesN<32>, // What Badge/Token should be used for votes
        threshold: u32,    // Voting threshold of token
    ) {
        env.storage().set(DataKey::Admin, admin);
        env.storage().set(DataKey::Token, token);
        env.storage().set(DataKey::Threshold, threshold);
        env.storage().set(DataKey::Status, 0 as u32);
    }

    // getStatus: Return status enum
    // NOTE: Status is currently hardcoded as u32 as a hack around enum issues
    pub fn get_status(env: Env) -> u32 {
        env.storage()
            .get(DataKey::Status)
            .expect("not initialized")
            .unwrap()
    }

    pub fn balance(env: Env) -> i128 {
        get_balance(&env, env.invoker())
    }

    // setStatus
    pub fn set_status(env: Env, status: u32) {
        if !(is_admin(&env, env.invoker())) {
            panic!("user is not an admin");
        }

        let cur_status: u32 = env.storage().get_unchecked(DataKey::Status).unwrap();

        if cur_status == status {
            return;
        }

        if cur_status == 1 {
            if status == 0 {
                panic!("Can't set status to Submission; Currently in Voting status");
            }
        }

        if status == 0 {
            delete_all_proposals(&env);
        }

        env.storage().set(DataKey::Status, status)
    }

    // get_admin
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .get(DataKey::Admin)
            .expect("not initialized")
            .unwrap()
    }

    // get_token
    pub fn get_token(env: Env) -> BytesN<32> {
        env.storage()
            .get(DataKey::Token)
            .expect("not initialized")
            .unwrap()
    }

    // get_thresh
    pub fn get_thresh(env: Env) -> u32 {
        get_threshold(&env)
    }

    // propose (AKA submitProposal): an account submits a proposal that can receive votes. One proposal per account.
    pub fn propose(env: Env, proposal_markdown: Bytes) {
        // Check if the contract is currently accepting submissions
        let current_status: u32 = Self::get_status(env.clone());
        if current_status != (Status::Submission as u32) {
            panic_with_error!(env.clone(), ContractError::NotAcceptingSubmissions);
        }

        if !is_eligible(&env, env.invoker()) {
            panic_with_error!(env.clone(), ContractError::UserNotEligible);
        }

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
        return proposals
            .get_unchecked(address.clone())
            .unwrap_or(Bytes::new(&env));
    }

    // eligible(id) (AKA verifyEligibility): checks if an account is eligible to voting
    pub fn eligible(env: Env, user: Address) -> bool {
        is_eligible(&env, user.into())
    }

    // vote(id) (AKA submitVote({id})): submit a vote for an existing proposal
    pub fn vote(env: Env, proposal_address: Address) {
        // Check if the contract is currently accepting submissions
        let current_status: u32 = Self::get_status(env.clone());
        if current_status != (Status::Voting as u32) {
            panic_with_error!(env.clone(), ContractError::NotAcceptingVotes);
        }

        if !is_eligible(&env, env.invoker()) {
            panic_with_error!(env.clone(), ContractError::UserNotEligible);
        }

        let proposals = Self::proposals(env.clone());
        if !proposals.contains_key(proposal_address.clone()) {
            panic_with_error!(env.clone(), ContractError::ProposalNotFound);
        }

        let users_votes_key = DataKey::UsersVotes;
        let mut users_votes: Map<Address, u32> = env
            .storage()
            .get(users_votes_key.clone())
            .unwrap_or(Ok(map![&env])) // If no value set, initialize it.
            .unwrap();

        let user_votes_count: u32 = users_votes
            .get(env.invoker())
            .unwrap_or(Ok(0)) // If no value set, initialize it.
            .unwrap();

        if user_votes_count >= MAX_USER_VOTE_COUNT {
            panic_with_error!(&env, ContractError::MaxUserVoteCountReached)
        }

        let proposals_votes_key = DataKey::PropsVotes;
        let mut proposals_votes: Map<Address, u32> = env
            .storage()
            .get(proposals_votes_key.clone())
            .unwrap_or(Ok(map![&env])) // If no value set, initialize it.
            .unwrap();

        let proposal_votes_count: u32 = proposals_votes
            .get(proposal_address.clone())
            .unwrap_or(Ok(0)) // If no value set, initialize it.
            .unwrap();

        // First make sure to update the proposal with the new vote
        proposals_votes.set(proposal_address.clone(), proposal_votes_count + 1);
        env.storage()
            .set(proposals_votes_key.clone(), proposals_votes);

        // Then finally update the user with the computed vote
        users_votes.set(env.invoker(), user_votes_count + 1);
        env.storage().set(users_votes_key.clone(), users_votes);
    }

    pub fn results(env: Env) -> Map<Address, u32> {
        let key = DataKey::PropsVotes;
        return env
            .storage()
            .get(key.clone())
            .unwrap_or(Ok(map![&env])) // If no value set, initialize it.
            .unwrap();
    }
}

mod errors;
mod tests;
