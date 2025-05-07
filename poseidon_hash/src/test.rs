#![cfg(test)]

use soroban_sdk::{vec, Env};

use crate::{
    operations::{digest::Digest, elem::Elem},
    PoseidonHashContract, PoseidonHashContractClient,
};

fn create_client(e: &Env) -> PoseidonHashContractClient {
    PoseidonHashContractClient::new(e, &e.register(PoseidonHashContract {}, ()))
}

#[test]
fn test() {
    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();

    let client = create_client(&env);
    let input = vec![
        &env,
        Elem::new(1),
        Elem::new(2),
        Elem::new(3),
        Elem::new(4),
        Elem::new(5),
        Elem::new(6),
        Elem::new(7),
        Elem::new(8),
    ];
    env.cost_estimate().budget().reset_default();
    let res = client.hash_elem_slice(&input);

    assert_eq!(
        res.as_words(),
        vec![
            &env, 1924395166, 735702495, 455179170, 55613122, 1240862023, 1674846984, 2009818198,
            1683428318
        ]
    );
    env.cost_estimate().budget().print();
}
