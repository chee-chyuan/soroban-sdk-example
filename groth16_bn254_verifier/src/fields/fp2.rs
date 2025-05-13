use soroban_sdk::contracttype;

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
}