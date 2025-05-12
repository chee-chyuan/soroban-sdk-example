use soroban_sdk::contracttype;

// in bn254 Fq is the same as Fp
// and so we can remove this file

#[contracttype]
#[derive(Clone)]
pub struct Fq {
    pub x: u128 // temporary because unit structures are not supported
}
