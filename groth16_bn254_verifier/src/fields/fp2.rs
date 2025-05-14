use soroban_sdk::{contracttype, Env};

use super::fp::Fp;
use ark_bn254::Fq2 as ArkFp2;

#[contracttype]
#[derive(Clone)]
pub struct Fp2 {
    pub c0: Fp,
    pub c1: Fp,
}

impl Fp2 {
    pub fn to_ark_fp2(&self) -> ArkFp2 {
        ArkFp2::new(self.c0.clone().to_ark_fp(), self.c1.clone().to_ark_fp())
    }

    pub fn from_ark_fp2(env: &Env, fp2: ArkFp2) -> Self {
        Fp2 {
            c0: Fp::from_ark_fp(env, fp2.c0),
            c1: Fp::from_ark_fp(env, fp2.c1),
        }
    }
}
