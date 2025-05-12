use soroban_sdk::contracttype;

use super::fp6_3over2::Fp6;

#[contracttype]
pub struct Fp12 {
    pub c0: Fp6,
    pub c1: Fp6,
}

pub type TargetField = Fp12;

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
