use soroban_sdk::contracttype;

use crate::fields::fp::Fp;

#[contracttype]
#[derive(Clone)]
pub struct G1Affine {
    pub x: Fp,
    pub y: Fp,
    pub infinity: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct G1Prepared(pub G1Affine);

impl G1Prepared {
    pub const fn is_zero(&self) -> bool {
        self.0.infinity
    }
}
