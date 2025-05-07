#![no_std]

mod operations;

use operations::{
    digest::{to_digest, Digest},
    elem::Elem,
    ext_elem::BabyBearExtElem,
    poseidon2_mix, BabyBearElem, BABY_BEAR_ELEM_ZERO, CELLS, CELLS_OUT, CELLS_RATE, DIGEST_WORDS,
};
use soroban_sdk::{contract, contractimpl, Env, Vec};

#[contract]
pub struct PoseidonHashContract;

#[contractimpl]
impl PoseidonHashContract {
    pub fn hash_pair(env: Env, a: Digest, b: Digest) -> Digest {
        let both = a
            .as_words()
            .iter()
            .chain(b.as_words().iter())
            .map(|w| BabyBearElem::new_raw(w));
        let mut both_vec = Vec::new(&env);
        for elem in both {
            both_vec.push_back(elem);
        }
        assert!(both_vec.len() as usize == DIGEST_WORDS * 2);
        for elem in both_vec.iter() {
            assert!(elem.is_reduced());
        }
        let unpadded = unpadded_hash2(both_vec);
        let digest = to_digest(&env, unpadded);
        digest
    }

    pub fn hash_elem_slice(env: Env, slice: Vec<BabyBearElem>) -> Digest {
        let unpadded = unpadded_hash2(slice);
        to_digest(&env, unpadded)
    }

    pub fn hash_ext_elem_slice(env: Env, slice: Vec<BabyBearExtElem>) -> Digest {
        let mut elements = Vec::new(&env);
        for elems in slice.iter() {
            for elem in elems.0.iter() {
                elements.push_back(elem);
            }
        }

        let unpadded = unpadded_hash2(elements);
        let digest = to_digest(&env, unpadded);
        digest
    }
}

fn unpadded_hash2(elems: Vec<BabyBearElem>) -> [BabyBearElem; CELLS_OUT] {
    let mut state = [Elem::new(0); CELLS];
    let mut count = 0;
    let mut unmixed = 0;
    for val in elems.iter() {
        state[unmixed] = val;
        count += 1;
        unmixed += 1;
        if unmixed == CELLS_RATE {
            poseidon2_mix(&mut state);
            unmixed = 0;
        }
    }
    if unmixed != 0 || count == 0 {
        // Zero pad to get a CELLS_RATE-aligned number of inputs
        for elem in state.iter_mut().take(CELLS_RATE).skip(unmixed) {
            *elem = BABY_BEAR_ELEM_ZERO;
        }
        poseidon2_mix(&mut state);
    }
    state.as_slice()[0..CELLS_OUT].try_into().unwrap()
}
