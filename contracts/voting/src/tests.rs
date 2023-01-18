#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Accounts, Env};

extern crate std;

#[test]
fn test_make_then_read_proposal_successfully() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, Voting);
    let client = VotingClient::new(&env, &contract_id);
    let user1 = env.accounts().generate();
    let user2 = env.accounts().generate();

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

#[test]
fn test_proposal_failure() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, Voting);
    let client = VotingClient::new(&env, &contract_id);
    let invoker_account = env.accounts().generate();

    // validate
    let address = Address::Account(invoker_account.clone());
    let result = client
        .with_source_account(&invoker_account)
        .try_proposal(&address);
    assert_eq!(result, Err(Ok(ContractError::ProposalNotFound)));
}

#[test]
fn test_make_then_read_all_proposals_successfully() {
    // setup
    let env = Env::default();
    let contract_id = env.register_contract(None, Voting);
    let client = VotingClient::new(&env, &contract_id);
    let user1 = env.accounts().generate();
    let user2 = env.accounts().generate();

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
