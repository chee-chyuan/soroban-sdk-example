use soroban_sdk::contracttype;

use crate::fields::fp::Fp;

#[contracttype]
#[derive(Clone)]
pub struct G1Projective {
    /// `X / Z` projection of the affine `X`
    pub x: Fp,
    /// `Y / Z` projection of the affine `Y`
    pub y: Fp,
    /// Projective multiplicative inverse. Will be `0` only at infinity.
    pub z: Fp,
}