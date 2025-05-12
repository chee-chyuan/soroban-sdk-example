use ark_bn254::G1Affine as ArkG1Affine;
use soroban_sdk::contracttype;

use crate::fields::fp::Fp;

#[contracttype]
#[derive(Clone)]
pub struct G1Affine {
    pub x: Fp,
    pub y: Fp,
    pub infinity: bool,
}

impl G1Affine {
    pub fn to_ark_g1_affine(&self) -> ArkG1Affine {
        ArkG1Affine::new(self.x.clone().to_ark_fp(), self.y.clone().to_ark_fp())
    }
}

#[contracttype]
#[derive(Clone)]
pub struct G1Prepared(pub G1Affine);

impl G1Prepared {
    pub const fn is_zero(&self) -> bool {
        self.0.infinity
    }
}
