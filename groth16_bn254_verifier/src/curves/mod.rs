use g1_affine::G1Prepared;
use g2_affine::G2Prepared;
use soroban_sdk::{Env, Vec};

pub mod g1_affine;
pub mod g1_projective;
pub mod g2_affine;

// impl BnConfig for Config {
//     const X: &'static [u64] = &[4965661367192848881];
//     /// `x` is positive.
//     const X_IS_NEGATIVE: bool = false;
//     const ATE_LOOP_COUNT: &'static [i8] = &[
//         0, 0, 0, 1, 0, 1, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, -1, 0, -1, 0, 0, 0, 1, 0, -1, 0, 0, 0,
//         0, -1, 0, 0, 1, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, -1, 0,
//         -1, 0, 0, 0, 1, 0, 1, 1,
//     ];

//     const TWIST_MUL_BY_Q_X: Fq2 = Fq2::new(
//         MontFp!("21575463638280843010398324269430826099269044274347216827212613867836435027261"),
//         MontFp!("10307601595873709700152284273816112264069230130616436755625194854815875713954"),
//     );
//     const TWIST_MUL_BY_Q_Y: Fq2 = Fq2::new(
//         MontFp!("2821565182194536844548159561693502659359617185244120367078079554186484126554"),
//         MontFp!("3505843767911556378687030309984248845540243509899259641013678093033130930403"),
//     );
//     const TWIST_TYPE: TwistType = TwistType::D;
//     type Fp = Fq;
//     type Fp2Config = Fq2Config;
//     type Fp6Config = Fq6Config;
//     type Fp12Config = Fq12Config;
//     type G1Config = g1::Config;
//     type G2Config = g2::Config;
// }

// pub struct MillerLoopOutput<P: Pairing>(pub P::TargetField);

pub fn multi_miller_loop(env: &Env, a: Vec<G1Prepared>, b: Vec<G2Prepared>) {
    if a.len() != b.len() {
        panic!("a and b must have the same length");
    }

    let zipped_a_b =
        a.iter()
            .zip(b.iter())
            .filter_map(|(p, q)| match !p.is_zero() && !q.is_zero() {
                true => Some((p, q.ell_coeffs)),
                false => None,
            });

    let mut pairs = Vec::new(env);
    for z in zipped_a_b {
        pairs.push_back(z);
    }

    // perform chunks_mut in a soroban compatible way
    let mut chunked = Vec::new(env);
    let mut i: u32 = 0;
    while i < pairs.len() {
        let chunk_size = if (pairs.len() - i) < 4 {
            pairs.len() - i
        } else {
            4
        };
        let chunk = pairs.slice(i..i + chunk_size);
        chunked.push_back(chunk);
        i += chunk_size;
    }

    // chunked.iter().map(|pairs| {
    //     let mut f = TargetField::one();
    //     //             let mut f = <Bn<Self> as Pairing>::TargetField::one();
    //     //             for i in (1..Self::ATE_LOOP_COUNT.len()).rev() {
    //     //                 if i != Self::ATE_LOOP_COUNT.len() - 1 {
    //     //                     f.square_in_place();
    //     //                 }

    //     //                 for (p, coeffs) in pairs.iter_mut() {
    //     //                     Bn::<Self>::ell(&mut f, &coeffs.next().unwrap(), &p.0);
    //     //                 }

    //     //                 let bit = Self::ATE_LOOP_COUNT[i - 1];
    //     //                 if bit == 1 || bit == -1 {
    //     //                     for (p, coeffs) in pairs.iter_mut() {
    //     //                         Bn::<Self>::ell(&mut f, &coeffs.next().unwrap(), &p.0);
    //     //                     }
    //     //                 }
    //     //             }
    //     //             f
    // });
}

// -> MillerLoopOutput<Bn<Self>> {}
// fn multi_miller_loop(
//     a: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
//     b: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
// ) -> MillerLoopOutput<Bn<Self>> {
//     let mut pairs = a
//         .into_iter()
//         .zip_eq(b)
//         .filter_map(|(p, q)| {
//             let (p, q) = (p.into(), q.into());
//             match !p.is_zero() && !q.is_zero() {
//                 true => Some((p, q.ell_coeffs.into_iter())),
//                 false => None,
//             }
//         })
//         .collect::<Vec<_>>();

//     let mut f = cfg_chunks_mut!(pairs, 4)
//         .map(|pairs| {
//             let mut f = <Bn<Self> as Pairing>::TargetField::one();
//             for i in (1..Self::ATE_LOOP_COUNT.len()).rev() {
//                 if i != Self::ATE_LOOP_COUNT.len() - 1 {
//                     f.square_in_place();
//                 }

//                 for (p, coeffs) in pairs.iter_mut() {
//                     Bn::<Self>::ell(&mut f, &coeffs.next().unwrap(), &p.0);
//                 }

//                 let bit = Self::ATE_LOOP_COUNT[i - 1];
//                 if bit == 1 || bit == -1 {
//                     for (p, coeffs) in pairs.iter_mut() {
//                         Bn::<Self>::ell(&mut f, &coeffs.next().unwrap(), &p.0);
//                     }
//                 }
//             }
//             f
//         })
//         .product::<<Bn<Self> as Pairing>::TargetField>();

//     if Self::X_IS_NEGATIVE {
//         f.cyclotomic_inverse_in_place();
//     }

//     for (p, coeffs) in &mut pairs {
//         Bn::<Self>::ell(&mut f, &coeffs.next().unwrap(), &p.0);
//     }

//     for (p, coeffs) in &mut pairs {
//         Bn::<Self>::ell(&mut f, &coeffs.next().unwrap(), &p.0);
//     }

//     MillerLoopOutput(f)
// }
