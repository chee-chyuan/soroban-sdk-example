use super::fp6_3over2::Fp6;
use ark_bn254::Fq12 as ArkFp12;
use soroban_sdk::contracttype;

#[contracttype]
pub struct Fp12 {
    pub c0: Fp6,
    pub c1: Fp6,
}

impl Fp12 {
    pub fn to_ark_fp12(&self) -> ArkFp12 {
        ArkFp12::new(self.c0.clone().to_ark_fp6(), self.c1.clone().to_ark_fp6())
    }
}