use soroban_sdk::{contracttype, Env};
use ark_bn254::Fq6 as ArkFp6;
use super::fp2::Fp2;

#[contracttype]
#[derive(Clone)]
pub struct Fp6 {
    pub c0: Fp2,
    pub c1: Fp2,
    pub c2: Fp2,
}

impl Fp6 {
    pub fn to_ark_fp6(&self) -> ArkFp6 {
        ArkFp6::new(self.c0.clone().to_ark_fp2(), self.c1.clone().to_ark_fp2(), self.c2.clone().to_ark_fp2())
    }

    pub fn from_ark_fp6(env: &Env, fp6: ArkFp6) -> Self {
        Fp6 {
            c0: Fp2::from_ark_fp2(env, fp6.c0),
            c1: Fp2::from_ark_fp2(env, fp6.c1),
            c2: Fp2::from_ark_fp2(env, fp6.c2),
        }
    }
}
