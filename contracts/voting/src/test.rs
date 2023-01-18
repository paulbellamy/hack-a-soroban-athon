#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Accounts, BytesN, symbol, vec};

#[test]
fn hello_test() {
    // we register the contract in a Soroban environment, and build a client we
    // can use to invoke the contract
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);

    // Next, we call `client.hello()`, supplying "Dev" as our `to` argument.
    let words = client.hello(&symbol!("Dev"));
    
    // We assert the contract must return a Vec that matches what we would
    // expect to receive from our contract: [Symbol("Hello"), Symbol("Dev")]
    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}

/// The first function, `test_store()`, will test the values that are being
/// stored by our contract. This is accomplished by generating a couple user
/// accounts, storing data as those users, and ensuring retrieved data matches
/// what we would expect it to be. We are also checking against a keypair that
/// hasn't stored any data, ensuring we receive Bytes of length 0 in return.
#[test]
fn test_store() {
    // Here we register the VotingContract contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);

    // We're generating two test users, `u1` and `u2` that will be the invokers
    // of the contract functions.
    let u1 = env.accounts().generate();
    let u2 = env.accounts().generate();

    // For our `u1` account, we store the `Bytes` represetation of "Hello
    // Soroban!" using the contract's `sbmt_prop()` function. We then use the
    // contracts `get_prop()` function to ensure we receive back the expected value.
    client
        .with_source_account(&u1)
        .sbmt_prop(&bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // hex value for "Hello Soroban!"
    assert_eq!(
        client.get_prop(&u1),
        bytes!(&env, 0x48656c6c6f20536f726f62616e21)
    );

    // Before storing any value as the `u2` account, we check to ensure `get_prop()`
    // returns 0 Bytes (i.e. the account has no data to get).
    assert_eq!(client.get_prop(&u2).len(), 0);

    // Now, as `u2`, we invoke the `sbmt_prop()` function, storing the `Bytes`
    // represetation of "Soroban Quest 2", asserting that `get_prop()` should return
    // the same back to us.
    client
        .with_source_account(&u2)
        .sbmt_prop(&bytes![&env, 0x536f726f62616e2051756573742032]); // hex value for "Soroban Quest 2"
    assert_eq!(
        client.get_prop(&u2),
        bytes![&env, 0x536f726f62616e2051756573742032]
    );

    // Of course, we expect that the data for `u1` has not been overwritten by
    // `u2` invoking the `put()` function.
    assert_eq!(
        client.get_prop(&u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}