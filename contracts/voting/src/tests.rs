#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Accounts, BytesN, Env};

extern crate std;

#[test]
fn test_make_then_read_proposal_successfully() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let user1 = env.accounts().generate();
    let user2 = env.accounts().generate();

    // initialize
    let admin = Address::Account(env.accounts().generate());
    client.initialize(&admin, &contract_id, &1);

    // test_propose (user1)
    let want_content1 = Bytes::from_slice(&env, b"Proposal text 1");
    client.with_source_account(&user1).propose(&want_content1);

    // test_propose (user2)
    let want_content2 = Bytes::from_slice(&env, b"Proposal text 2");
    client.with_source_account(&user2).propose(&want_content2);

    // validate (user1)
    let address1 = Address::Account(user1.clone());
    let mut got_content1 = client.with_source_account(&user1).proposal(&address1);
    assert_eq!(want_content1, got_content1);

    // validate (user2)
    let address2 = Address::Account(user2.clone());
    let got_content2 = client.with_source_account(&user2).proposal(&address2);
    assert_eq!(want_content2, got_content2);

    // override proposal (user1)
    let want_content_new = Bytes::from_slice(&env, b"New proposal text 1");
    client
        .with_source_account(&user1)
        .propose(&want_content_new);

    // validate new value (user1)
    got_content1 = client.with_source_account(&user1).proposal(&address1);
    assert_eq!(want_content_new, got_content1);
}

/// For the first test we are going to test the propose() behavior when it is
/// invoked from another contract (i.e. not an user). So, we are creating a
/// very simple Smart Contract here, that we can use as the caller (invoker).
pub struct CallerContract;

#[contractimpl]
impl CallerContract {
    pub fn try_prop(env: Env, contract_id: BytesN<32>, proposal_markdown: Bytes) {
        let voting_contract_client = VotingContractClient::new(&env, contract_id);
        voting_contract_client.propose(&proposal_markdown);
    }
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_make_proposal_failure_not_an_user() {
    // setup
    let env = Env::default();
    let voting_contract_id = env.register_contract(None, VotingContract);
    let voting_contract_client = VotingContractClient::new(&env, &voting_contract_id);

    let caller_contract_id = env.register_contract(None, CallerContract);
    let caller_contract_client = CallerContractClient::new(&env, caller_contract_id.clone());

    // initialize
    let admin = Address::Account(env.accounts().generate());
    voting_contract_client.initialize(&admin, &caller_contract_id.clone(), &1);
    
    // test_propose (calling as a contract)
    let want_content1 = Bytes::from_slice(&env, b"Contract caller proposal");
    caller_contract_client.try_prop(&voting_contract_id.clone(), &want_content1);
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_make_proposal_failure_too_short() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let user1 = env.accounts().generate();

    // initialize
    let admin = Address::Account(env.accounts().generate());
    client.initialize(&admin, &contract_id, &1);

    // test_propose (user1)
    let want_content1 = Bytes::from_slice(&env, b"too short");
    client.with_source_account(&user1).propose(&want_content1);
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_make_proposal_failure_too_long() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let user1 = env.accounts().generate();

    // initialize
    let admin = Address::Account(env.accounts().generate());
    client.initialize(&admin, &contract_id, &1);

    // test_propose (user1)
    let mut want_content1 = Bytes::from_slice(&env, b"too long");
    for _ in 0..=400 {
        let new = Bytes::from_slice(&env, b"xxxxx");
        want_content1.append(&new)
    }
    client.with_source_account(&user1).propose(&want_content1);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_proposal_not_found_failure() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let invoker_account = env.accounts().generate();

    // validate "proposal not found"
    let address = Address::Account(invoker_account.clone());
    client
        .with_source_account(&invoker_account)
        .proposal(&address);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_proposal_fails_if_not_accepting_proposals() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let proposer1 = env.accounts().generate();
    
    // initialize
    let admin = env.accounts().generate();
    client.initialize(&Address::Account(admin.clone()), &contract_id, &1);
    // set status to Voting
    client.with_source_account(&admin.clone()).set_status(&(Status::Voting as u32));

    // proposer1 adds a proposal:
    let want_content1 = Bytes::from_slice(&env, b"Proposal text 1");
    client.with_source_account(&proposer1).propose(&want_content1);
}

#[test]
fn test_make_then_read_all_proposals_successfully() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let user1 = env.accounts().generate();
    let user2 = env.accounts().generate();

    // initialize
    let admin = Address::Account(env.accounts().generate());
    client.initialize(&admin, &contract_id, &1);

    // empty proposals:
    let mut got_proposals = client.with_source_account(&user1).proposals();
    let mut want_proposals = map![&env];
    assert_eq!(want_proposals, got_proposals);

    // user1 adds a proposal:
    let want_content1 = Bytes::from_slice(&env, b"Proposal text 1");
    client.with_source_account(&user1).propose(&want_content1);
    got_proposals = client.with_source_account(&user1).proposals();
    want_proposals = map![&env];
    want_proposals.set(Address::Account(user1.clone()), want_content1.clone());
    assert_eq!(want_proposals, got_proposals);

    // user2 adds a proposal:
    let want_content2 = Bytes::from_slice(&env, b"Proposal text 2");
    client.with_source_account(&user2).propose(&want_content2);
    got_proposals = client.with_source_account(&user2).proposals();
    want_proposals = map![&env];
    want_proposals.set(Address::Account(user1.clone()), want_content1.clone());
    want_proposals.set(Address::Account(user2.clone()), want_content2.clone());
    assert_eq!(want_proposals, got_proposals);

    // a new proposal from user1 will just override its old proposal
    let want_content1_new = Bytes::from_slice(&env, b"New proposal text 1");
    client
        .with_source_account(&user1)
        .propose(&want_content1_new);
    got_proposals = client.with_source_account(&user1).proposals();
    want_proposals = map![&env];
    want_proposals.set(Address::Account(user1.clone()), want_content1_new.clone());
    want_proposals.set(Address::Account(user2.clone()), want_content2.clone());
    assert_eq!(want_proposals, got_proposals);
}

#[test]
fn test_is_eligible() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let user1 = env.accounts().generate();

    // validate
    let is_eligigle = client.with_source_account(&user1).eligible();
    assert_eq!(is_eligigle, true);
}

#[test]
fn test_vote() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let proposer1 = env.accounts().generate();
    let proposer2 = env.accounts().generate();

    // initialize
    let admin = env.accounts().generate();
    client.initialize(&Address::Account(admin.clone()), &contract_id, &1);

    // proposer1 adds a proposal:
    let want_content1 = Bytes::from_slice(&env, b"Proposal text 1");
    client.with_source_account(&proposer1).propose(&want_content1);

    // proposer2 adds a proposal:
    let want_content2 = Bytes::from_slice(&env, b"Proposal text 2");
    client.with_source_account(&proposer2).propose(&want_content2);

    // set status to Voting
    client.with_source_account(&admin.clone()).set_status(&(Status::Voting as u32));

    // user votes in a proposal
    let user = env.accounts().generate();
    client
        .with_source_account(&user)
        .vote(&Address::Account(proposer2.clone()));

    let results = client.results();

    assert_eq!(results.get(Address::Account(proposer2.clone())).unwrap_or(Ok(0)).unwrap(), 1);
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn test_vote_above_max_count() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let proposer1 = env.accounts().generate();
    let proposer2 = env.accounts().generate();
    
    // initialize
    let admin = env.accounts().generate();
    client.initialize(&Address::Account(admin.clone()), &contract_id, &1);

    // proposer1 adds a proposal:
    let want_content1 = Bytes::from_slice(&env, b"Proposal text 1");
    client.with_source_account(&proposer1).propose(&want_content1);

    // proposer2 adds a proposal:
    let want_content2 = Bytes::from_slice(&env, b"Proposal text 2");
    client.with_source_account(&proposer2).propose(&want_content2);

    // set status to Voting
    client.with_source_account(&admin.clone()).set_status(&(Status::Voting as u32));

    // user votes in a proposal
    let user = env.accounts().generate();
    client
        .with_source_account(&user)
        .vote(&Address::Account(proposer1.clone()));

    // user tries to vote in another proposal (max count is currently 1 so should panic)
    client
        .with_source_account(&user)
        .vote(&Address::Account(proposer2.clone()));
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn test_vote_fails_if_vote_is_disabled() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let proposer1 = env.accounts().generate();
    
    // initialize
    let admin = env.accounts().generate();
    client.initialize(&Address::Account(admin.clone()), &contract_id, &1);

    // proposer1 adds a proposal:
    let want_content1 = Bytes::from_slice(&env, b"Proposal text 1");
    client.with_source_account(&proposer1).propose(&want_content1);

    // user votes in a proposal
    let user = env.accounts().generate();
    client
        .with_source_account(&user)
        .vote(&Address::Account(proposer1.clone()));
}

#[test]
fn test_results() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let proposer1 = env.accounts().generate();
    let proposer2 = env.accounts().generate();
    let proposer3 = env.accounts().generate();

    // initialize
    let admin = env.accounts().generate();
    client.initialize(&Address::Account(admin.clone()), &contract_id, &1);

    // proposer1 adds a proposal:
    let want_content1 = Bytes::from_slice(&env, b"Proposal text 1");
    client.with_source_account(&proposer1).propose(&want_content1);

    // proposer2 adds a proposal:
    let want_content2 = Bytes::from_slice(&env, b"Proposal text 2");
    client.with_source_account(&proposer2).propose(&want_content2);

    // proposer3 adds a proposal:
    let want_content3 = Bytes::from_slice(&env, b"Proposal text 3");
    client.with_source_account(&proposer3).propose(&want_content3);

    // set status to Voting
    client.with_source_account(&admin.clone()).set_status(&(Status::Voting as u32));

    // submit some votes for each proposal
    for _ in 0..2 {
        client
        .with_source_account(&env.accounts().generate())
        .vote(&Address::Account(proposer1.clone()));
    }

    for _ in 0..4 {
        client
        .with_source_account(&env.accounts().generate())
        .vote(&Address::Account(proposer2.clone()));
    }

    for _ in 0..6 {
        client
        .with_source_account(&env.accounts().generate())
        .vote(&Address::Account(proposer3.clone()));
    }

    let results = client.results();

    assert_eq!(results.get(Address::Account(proposer1.clone())).unwrap_or(Ok(0)).unwrap(), 2);
    assert_eq!(results.get(Address::Account(proposer2.clone())).unwrap_or(Ok(0)).unwrap(), 4);
    assert_eq!(results.get(Address::Account(proposer3.clone())).unwrap_or(Ok(0)).unwrap(), 6);

    std::println!("# # # # # Voting Results: {:#?}", results);

}

#[test]
fn test_initialize_contract() {
     // setup
     let env = Env::default();
     let contract_id = env.register_contract(None, VotingContract);
     let client = VotingContractClient::new(&env, &contract_id);
     let invoker_account = env.accounts().generate();
     let address = Address::Account(invoker_account.clone());

     // test initialize
     client.initialize(&address, &contract_id, &1);

    // validate initialization
    let admin: Address = client.get_admin();
    assert_eq!(admin, address);

    let token: BytesN<32> = client.get_token();
    assert_eq!(token, contract_id);

    let threshold: u32 = client.get_thresh();
    assert_eq!(threshold, 1);

    let status: u32 = client.get_status();
    assert_eq!(status, 0);
}

#[test]
fn test_set_status() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);
    let invoker_account = env.accounts().generate();
    let address = Address::Account(invoker_account.clone());
    client.initialize(&address, &contract_id, &1);

    // test set_status
    client.with_source_account(&invoker_account).set_status(&1);

    let status: u32 = client.get_status();
    assert_eq!(status, 1);
}
