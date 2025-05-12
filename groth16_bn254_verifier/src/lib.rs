#![no_std]
use curves::g1_projective::G1Projective;
use groth16::data_structure::{PreparedVerifyingKey, Proof};
use soroban_sdk::{contract, contractimpl};

mod curves;
mod fields;
mod groth16;
mod verifier;
mod helper;

#[contract]
pub struct Groth16BN254Verifier;

#[contractimpl]
impl Groth16BN254Verifier {
    // pub fn verify_proof_with_prepared_inputs(
    //     pvk: &PreparedVerifyingKey<E>,
    //     proof: &Proof<E>,
    //     prepared_inputs: &E::G1,
    // ) -> R1CSResult<bool> {

    // verify_proof_with_prepared_inputs
    pub fn verify_with_prepared_inputs(
        pvk: PreparedVerifyingKey,
        proof: Proof,
        prepared_inputs: G1Projective,
    ) -> bool {
        // let qap = E::multi_miller_loop(
        //     [
        //         <E::G1Affine as Into<E::G1Prepared>>::into(proof.a),
        //         prepared_inputs.into_affine().into(),
        //         proof.c.into(),
        //     ],
        //     [
        //         proof.b.into(),
        //         pvk.gamma_g2_neg_pc.clone(),
        //         pvk.delta_g2_neg_pc.clone(),
        //     ],
        // );

        // let test = E::final_exponentiation(qap).ok_or(SynthesisError::UnexpectedIdentity)?;

        // Ok(test.0 == pvk.alpha_g1_beta_g2)
        true
    }
}
