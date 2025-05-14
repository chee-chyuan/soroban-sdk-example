use ark_bn254::{Bn254, G2Affine as ArkG2Affine};
use ark_ec::models::bn::g2::G2Prepared as ArkG2Prepared;
use ark_groth16::{
    Groth16, PreparedVerifyingKey as ArkPreparedVerifyingKey, Proof as ArkProof,
    VerifyingKey as ArkVerifyingKey,
};
use soroban_sdk::{contracttype, vec, Env, Vec};

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

impl VerifyingKey {
    pub fn to_ark_vk(&self) -> ArkVerifyingKey<Bn254> {
        ArkVerifyingKey {
            alpha_g1: self.alpha_g1.to_ark_g1_affine(),
            beta_g2: self.beta_g2.to_ark_g2_affine(),
            gamma_g2: self.gamma_g2.to_ark_g2_affine(),
            delta_g2: self.delta_g2.to_ark_g2_affine(),
            gamma_abc_g1: self
                .gamma_abc_g1
                .iter()
                .map(|g1| g1.to_ark_g1_affine())
                .collect(),
        }
    }

    pub fn from_ark_vk(env: &Env, vk: ArkVerifyingKey<Bn254>) -> Self {
        // iter doenst exist in soroban's Vec
        let gamma_abc_g1_iter = vk
            .gamma_abc_g1
            .iter()
            .map(|g1| G1Affine::from_ark_g1_affine(env, *g1));

        let mut gamma_abc_g1: Vec<G1Affine> = vec![env];
        for g1 in gamma_abc_g1_iter {
            gamma_abc_g1.push_back(g1);
        }

        VerifyingKey {
            alpha_g1: G1Affine::from_ark_g1_affine(env, vk.alpha_g1),
            beta_g2: G2Affine::from_ark_g2_affine(env, vk.beta_g2),
            gamma_g2: G2Affine::from_ark_g2_affine(env, vk.gamma_g2),
            delta_g2: G2Affine::from_ark_g2_affine(env, vk.delta_g2),
            gamma_abc_g1,
        }
    }
}

#[contracttype]
pub struct PreparedVerifyingKey {
    /// The unprepared verification key.
    pub vk: VerifyingKey,
    /// The element `e(alpha * G, beta * H)` in `E::GT`.
    pub alpha_g1_beta_g2: TargetField,
    // /// The element `- gamma * H` in `E::G2`, prepared for use in pairings.
    // pub gamma_g2_neg_pc: G2Prepared,
    // /// The element `- delta * H` in `E::G2`, prepared for use in pairings.
    // pub delta_g2_neg_pc: G2Prepared,
    pub gamma_g2_neg_pc: G2Prepared,
    pub delta_g2_neg_pc: G2Prepared,
}

impl PreparedVerifyingKey {
    pub fn to_ark_pvk(&self) -> ArkPreparedVerifyingKey<Bn254> {
        // let gamma_g2_neg_pc = ArkG2Prepared::from(self.gamma_g2_neg_pc.to_ark_g2_affine());
        // let delta_g2_neg_pc = ArkG2Prepared::from(self.delta_g2_neg_pc.to_ark_g2_affine());
        ArkPreparedVerifyingKey {
            vk: self.vk.to_ark_vk(),
            alpha_g1_beta_g2: self.alpha_g1_beta_g2.to_ark_fp12(),
            gamma_g2_neg_pc: self.gamma_g2_neg_pc.to_ark_g2_prepared(),
            delta_g2_neg_pc: self.delta_g2_neg_pc.to_ark_g2_prepared(),
        }
    }

    pub fn from_ark_pvk(env: &Env, pvk: ArkPreparedVerifyingKey<Bn254>) -> Self {
        // let gamma_g2_neg_pc_prepared = pvk.gamma_g2_neg_pc;
        // let gamma_g2_neg_pc = ArkG2Affine::from(gamma_g2_neg_pc_prepared);

        PreparedVerifyingKey {
            vk: VerifyingKey::from_ark_vk(env, pvk.vk),
            alpha_g1_beta_g2: Fp12::from_ark_fp12(env, pvk.alpha_g1_beta_g2),
            gamma_g2_neg_pc: G2Prepared::from_ark_g2_prepared(env, pvk.gamma_g2_neg_pc),
            delta_g2_neg_pc: G2Prepared::from_ark_g2_prepared(env, pvk.delta_g2_neg_pc),
        }
    }
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

impl Proof {
    pub fn to_ark_proof(&self) -> ArkProof<Bn254> {
        ArkProof {
            a: self.a.to_ark_g1_affine(),
            b: self.b.to_ark_g2_affine(),
            c: self.c.to_ark_g1_affine(),
        }
    }

    pub fn from_ark_proof(env: &Env, proof: ArkProof<Bn254>) -> Self {
        Proof {
            a: G1Affine::from_ark_g1_affine(env, proof.a),
            b: G2Affine::from_ark_g2_affine(env, proof.b),
            c: G1Affine::from_ark_g1_affine(env, proof.c),
        }
    }
}
