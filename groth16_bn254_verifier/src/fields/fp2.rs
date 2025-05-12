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
    // pub fn one() -> Self {
    //     Fp2 {
    //         c0: Fp::one(),
    //         c1: Fp::zero(),
    //     }
    // }

    // pub fn zero() -> Self {
    //     Fp2 {
    //         c0: Fp::zero(),
    //         c1: Fp::zero(),
    //     }
    // }
}
// impl<P: QuadExtConfig> Zero for QuadExtField<P> {
//     fn zero() -> Self {
//         QuadExtField::new(P::BaseField::zero(), P::BaseField::zero())
//     }

//     fn is_zero(&self) -> bool {
//         self.c0.is_zero() && self.c1.is_zero()
//     }
// }

// impl<P: QuadExtConfig> One for QuadExtField<P> {
//     fn one() -> Self {
//         QuadExtField::new(P::BaseField::one(), P::BaseField::zero())
//     }

//     fn is_one(&self) -> bool {
//         self.c0.is_one() && self.c1.is_zero()
//     }
// }
