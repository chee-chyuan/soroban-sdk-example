use soroban_sdk::contracttype;
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
}