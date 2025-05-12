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

// impl Fp6 {
//     pub fn one() -> Self {
//         Fp6 {
//             c0: Fp2::one(),
//             c1: Fp2::zero(),
//             c2: Fp2::zero(),
//         }
//     }

//     pub fn zero() -> Self {
//         Fp6 {
//             c0: Fp2::zero(),
//             c1: Fp2::zero(),
//             c2: Fp2::zero(),
//         }
//     }
// }

// impl<P: CubicExtConfig> Zero for CubicExtField<P> {
//     fn zero() -> Self {
//         Self::new(P::BaseField::ZERO, P::BaseField::ZERO, P::BaseField::ZERO)
//     }

//     fn is_zero(&self) -> bool {
//         self.c0.is_zero() && self.c1.is_zero() && self.c2.is_zero()
//     }
// }

// impl<P: CubicExtConfig> One for CubicExtField<P> {
//     fn one() -> Self {
//         Self::new(P::BaseField::ONE, P::BaseField::ZERO, P::BaseField::ZERO)
//     }

//     fn is_one(&self) -> bool {
//         self.c0.is_one() && self.c1.is_zero() && self.c2.is_zero()
//     }
// }
