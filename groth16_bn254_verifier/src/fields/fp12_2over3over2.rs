use super::fp6_3over2::Fp6;
use ark_bn254::Fq12 as ArkFp12;
use soroban_sdk::{contracttype, Env};

#[contracttype]
pub struct Fp12 {
    pub c0: Fp6,
    pub c1: Fp6,
}

impl Fp12 {
    pub fn to_ark_fp12(&self) -> ArkFp12 {
        ArkFp12::new(self.c0.clone().to_ark_fp6(), self.c1.clone().to_ark_fp6())
    }

    pub fn from_ark_fp12(env: &Env, fp12: ArkFp12) -> Self {
        Fp12 {
            c0: Fp6::from_ark_fp6(env, fp12.c0),
            c1: Fp6::from_ark_fp6(env, fp12.c1),
        }
    }
}
