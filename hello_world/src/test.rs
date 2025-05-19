#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}




// stellar contract upload \
//   --network testnet \
//   --source SBQZIRY4U6HEWA7INGHM2ZJIUZ2NUOUEAKEQQKV67GYD4S2WWAW66GDY \
//   --wasm target/wasm32-unknown-unknown/release/soroban_hello_world_contract.wasm 

// wasm hash
// fec2819684a9c2964614b769ef881c66848af7ef0eabf1ddb968fc0fef36b11e

// stellar contract deploy \
//   --wasm-hash fec2819684a9c2964614b769ef881c66848af7ef0eabf1ddb968fc0fef36b11e \
//   --source SBQZIRY4U6HEWA7INGHM2ZJIUZ2NUOUEAKEQQKV67GYD4S2WWAW66GDY \
//   --network testnet \
//   --alias hello_world_test

// contract id
// https://stellar.expert/explorer/testnet/contract/CCR7I27QJNKRAQOIR6L2GRDPEBJP64CCQFGXXSXOQCQGY7K6X7DRMRKN
// CCR7I27QJNKRAQOIR6L2GRDPEBJP64CCQFGXXSXOQCQGY7K6X7DRMRKN