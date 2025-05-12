use ark_bn254::G2Affine as ArkG2Affine;
use soroban_sdk::{contracttype, Vec};

use crate::fields::fp2::Fp2;

#[contracttype]
pub struct G2Affine {
    pub x: Fp2,
    pub y: Fp2,
    pub infinity: bool,
}

impl G2Affine {
    pub fn to_ark_g2_affine(&self) -> ArkG2Affine {
        ArkG2Affine::new(self.x.clone().to_ark_fp2(), self.y.clone().to_ark_fp2())
    }
}

#[contracttype]
#[derive(Clone)]
pub struct G2Prepared {
    /// Stores the coefficients of the line evaluations as calculated in
    /// <https://eprint.iacr.org/2013/722.pdf>
    pub ell_coeffs: Vec<EllCoeff>,
    pub infinity: bool,
}

impl G2Prepared {
    pub const fn is_zero(&self) -> bool {
        self.infinity
    }
}

#[contracttype]
#[derive(Clone)]
pub struct EllCoeff {
    pub c0: Fp2,
    pub c1: Fp2,
    pub c2: Fp2,
}
