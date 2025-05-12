use soroban_sdk::{contracttype, Vec};

use crate::{
    curves::{
        g1_affine::G1Affine,
        g2_affine::{G2Affine, G2Prepared},
    },
    fields::fp12_2over3over2::Fp12,
};

#[contracttype]
pub struct VerifyingKey {
    /// The `alpha * G`, where `G` is the generator of `E::G1`.
    pub alpha_g1: G1Affine,
    /// The `alpha * H`, where `H` is the generator of `E::G2`.
    pub beta_g2: G2Affine,
    /// The `gamma * H`, where `H` is the generator of `E::G2`.
    pub gamma_g2: G2Affine,
    /// The `delta * H`, where `H` is the generator of `E::G2`.
    pub delta_g2: G2Affine,
    /// The `gamma^{-1} * (beta * a_i + alpha * b_i + c_i) * H`, where `H` is
    /// the generator of `E::G1`.
    pub gamma_abc_g1: Vec<G1Affine>,
}

#[contracttype]
pub struct PreparedVerifyingKey {
    /// The unprepared verification key.
    pub vk: VerifyingKey,
    /// The element `e(alpha * G, beta * H)` in `E::GT`.
    pub alpha_g1_beta_g2: TargetField,
    /// The element `- gamma * H` in `E::G2`, prepared for use in pairings.
    pub gamma_g2_neg_pc: G2Prepared,
    /// The element `- delta * H` in `E::G2`, prepared for use in pairings.
    pub delta_g2_neg_pc: G2Prepared,
}

type TargetField = Fp12;

#[contracttype]
pub struct Proof {
    /// The `A` element in `G1`.
    pub a: G1Affine,
    /// The `B` element in `G2`.
    pub b: G2Affine,
    /// The `C` element in `G1`.
    pub c: G1Affine,
}
