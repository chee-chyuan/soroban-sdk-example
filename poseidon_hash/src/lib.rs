#![no_std]

mod operations;

use operations::{
    elem::Elem, ext_elem::BabyBearExtElem, poseidon2_mix, BabyBearElem, BABY_BEAR_ELEM_ZERO, CELLS,
    CELLS_OUT, CELLS_RATE, DIGEST_WORDS,
};
use soroban_sdk::{contract, contractimpl, Env, Vec};

#[contract]
pub struct PoseidonHashContract;

pub struct Digest([u32; DIGEST_WORDS]);

#[contractimpl]
impl PoseidonHashContract {
    // pub fn hash(env: Env) {}

    // fn hash_pair(&self, a: &Digest, b: &Digest) -> Box<Digest> {
    //     let both: Vec<BabyBearElem> = a
    //         .as_words()
    //         .iter()
    //         .chain(b.as_words())
    //         .map(|w| BabyBearElem::new_raw(*w))
    //         .collect();
    //     assert!(both.len() == DIGEST_WORDS * 2);
    //     for elem in &both {
    //         assert!(elem.is_reduced());
    //     }
    //     to_digest(unpadded_hash(both.iter()))
    // }

    // fn hash_elem_slice(&self, slice: &[BabyBearElem]) -> Box<Digest> {
    //     to_digest(unpadded_hash(slice.iter()))
    // }

    // fn hash_ext_elem_slice(&self, slice: &[BabyBearExtElem]) -> Box<Digest> {
    //     to_digest(unpadded_hash(
    //         slice.iter().flat_map(|ee| ee.subelems().iter()),
    //     ))
    // }
    pub fn hash_ext_elem_slice(env: Env, slice: Vec<BabyBearExtElem>) //-> Digest
    {
        // to_digest(unpadded_hash(
        //     slice.iter().flat_map(|ee| ee.subelems().iter()),
        // ))

        // let elements: Vec<&BabyBearElem> = slice.iter().flat_map(|ee| ee.elems().iter()).collect();
        // to_digest(unpadded_hash(elements.into_iter()))
    }
}

fn to_digest(elems: [BabyBearElem; CELLS_OUT]) -> Digest {
    let mut state: [u32; DIGEST_WORDS] = [0; DIGEST_WORDS];
    for i in 0..DIGEST_WORDS {
        state[i] = elems[i].as_u32_montgomery();
    }

    Digest(state)
}

fn unpadded_hash<'a, I>(iter: I) -> [BabyBearElem; CELLS_OUT]
where
    I: Iterator<Item = &'a BabyBearElem>,
{
    let mut state = [Elem::new(0); CELLS];
    let mut count = 0;
    let mut unmixed = 0;
    for val in iter {
        state[unmixed] = *val;
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
// pub fn unpadded_hash<'a, I>(iter: I) -> [BabyBearElem; CELLS_OUT]
// where
//     I: Iterator<Item = &'a BabyBearElem>,
// {
//     let mut state = [BabyBearElem::ZERO; CELLS];
//     let mut count = 0;
//     let mut unmixed = 0;
//     for val in iter {
//         state[unmixed] = *val;
//         count += 1;
//         unmixed += 1;
//         if unmixed == CELLS_RATE {
//             poseidon2_mix(&mut state);
//             unmixed = 0;
//         }
//     }
//     if unmixed != 0 || count == 0 {
//         // Zero pad to get a CELLS_RATE-aligned number of inputs
//         for elem in state.iter_mut().take(CELLS_RATE).skip(unmixed) {
//             *elem = BabyBearElem::ZERO;
//         }
//         poseidon2_mix(&mut state);
//     }
//     state.as_slice()[0..CELLS_OUT].try_into().unwrap()
// }
