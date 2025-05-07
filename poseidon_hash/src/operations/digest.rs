use soroban_sdk::{contracttype, Env, Vec};

use super::{BabyBearElem, CELLS_OUT, DIGEST_WORDS};

#[contracttype]
pub struct Digest(Vec<u32>);

impl Digest {
    pub fn as_words(&self) -> Vec<u32> {
        self.0.clone()
    }
}

pub fn to_digest(env: &Env, elems: [BabyBearElem; CELLS_OUT]) -> Digest {
    let mut state: [u32; DIGEST_WORDS] = [0; DIGEST_WORDS];
    for i in 0..DIGEST_WORDS {
        state[i] = elems[i].as_u32_montgomery();
    }

    Digest(Vec::from_array(env, state))
}
