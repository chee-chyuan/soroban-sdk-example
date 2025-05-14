use ark_bn254::{Config as Bn254Config, G2Affine as ArkG2Affine};
use ark_ec::bn::g2::{EllCoeff as ArkEllCoeff, G2Prepared as ArkG2Prepared};
use soroban_sdk::{contracttype, vec, Env, Vec};

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

    pub fn from_ark_g2_affine(env: &Env, g2_affine: ArkG2Affine) -> Self {
        G2Affine {
            x: Fp2::from_ark_fp2(env, g2_affine.x),
            y: Fp2::from_ark_fp2(env, g2_affine.y),
            infinity: g2_affine.infinity,
        }
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

    pub fn to_ark_g2_prepared(&self) -> ArkG2Prepared<Bn254Config> {
        let ell_coeffs = self
            .ell_coeffs
            .iter()
            .map(|coeff| coeff.to_ark_ell_coeff())
            .collect();

        ArkG2Prepared::<Bn254Config> {
            ell_coeffs,
            infinity: self.infinity,
        }
    }

    pub fn from_ark_g2_prepared(env: &Env, g2_prepared: ArkG2Prepared<Bn254Config>) -> Self {
        let ell_coeffs_iter = g2_prepared
            .ell_coeffs
            .iter()
            .map(|coeff| EllCoeff::from_ark_ell_coeff(env, *coeff));

        let mut ell_coeffs: Vec<EllCoeff> = vec![env];
        for coeff in ell_coeffs_iter {
            ell_coeffs.push_back(coeff);
        }

        G2Prepared {
            ell_coeffs,
            infinity: g2_prepared.infinity,
        }
    }
}

#[contracttype]
#[derive(Clone)]
pub struct EllCoeff {
    pub c0: Fp2,
    pub c1: Fp2,
    pub c2: Fp2,
}

impl EllCoeff {
    pub fn from_ark_ell_coeff(env: &Env, coeff: ArkEllCoeff<Bn254Config>) -> Self {
        Self {
            c0: Fp2::from_ark_fp2(env, coeff.0),
            c1: Fp2::from_ark_fp2(env, coeff.1),
            c2: Fp2::from_ark_fp2(env, coeff.2),
        }
    }

    pub fn to_ark_ell_coeff(&self) -> ArkEllCoeff<Bn254Config> {
        (
            self.c0.clone().to_ark_fp2(),
            self.c1.clone().to_ark_fp2(),
            self.c2.clone().to_ark_fp2(),
        )
    }
}
