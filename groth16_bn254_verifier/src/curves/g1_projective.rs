use ark_bn254::G1Projective as ArkG1Projective;
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

impl G1Projective {
    pub fn to_ark_g1_projective(&self) -> ArkG1Projective {
        ArkG1Projective::new(
            self.x.clone().to_ark_fp(),
            self.y.clone().to_ark_fp(),
            self.z.clone().to_ark_fp(),
        )
    }
}
