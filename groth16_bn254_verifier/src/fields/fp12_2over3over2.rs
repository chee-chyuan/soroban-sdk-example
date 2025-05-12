use super::fp6_3over2::Fp6;
use ark_bn254::Fq12 as ArkFp12;
use soroban_sdk::contracttype;

#[contracttype]
pub struct Fp12 {
    pub c0: Fp6,
    pub c1: Fp6,
}

pub type TargetField = Fp12;

impl Fp12 {
    pub fn to_ark_fp12(&self) -> ArkFp12 {
        ArkFp12::new(self.c0.clone().to_ark_fp6(), self.c1.clone().to_ark_fp6())
    }
}

// impl Fp12 {
//     pub fn one() -> Self {
//         Fp12 {
//             c0: Fp6::one(),
//             c1: Fp6::zero(),
//         }
//     }
// }

// pub struct QuadExtField<P: QuadExtConfig> {
//     /// Coefficient `c0` in the representation of the field element `c = c0 + c1 * X`
//     pub c0: P::BaseField,
//     /// Coefficient `c1` in the representation of the field element `c = c0 + c1 * X`
//     pub c1: P::BaseField,
// }

// fn one() -> Self {
//     QuadExtField::new(P::BaseField::one(), P::BaseField::zero())
// }
