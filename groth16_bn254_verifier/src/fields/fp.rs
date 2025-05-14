use ark_bn254::Fq;
use ark_ff::{BigInt, PrimeField};
use soroban_sdk::{contracttype, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Fp {
    pub bigint: Vec<u64>,
}

impl Fp {
    pub const ZERO: [u64; 4] = [0u64; 4];
    pub const N: usize = 4;

    pub fn zero(env: &Env) -> Self {
        Fp {
            bigint: Vec::from_array(env, Self::ZERO),
        }
    }

    pub fn to_ark_fp(self) -> Fq {
        let bigint = BigInt([
            self.bigint.get(0).unwrap(),
            self.bigint.get(1).unwrap(),
            self.bigint.get(2).unwrap(),
            self.bigint.get(3).unwrap(),
        ]);

        Fq::from_bigint(bigint).unwrap()
    }

    pub fn from_ark_fp(env: &Env, fp: Fq) -> Self {
        let bigint = fp.into_bigint();
        Fp {
            bigint: Vec::from_array(env, bigint.0),
        }
    }
}