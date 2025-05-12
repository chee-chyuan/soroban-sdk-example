#![no_std]
use ark_bn254::{Bn254, G1Affine};
use ark_groth16::{Groth16, VerifyingKey};
use curves::g1_affine::G1Affine as G1AffineSoroban;
use curves::g1_projective::G1Projective as G1ProjectiveSoroban;
use groth16::data_structure::{
    PreparedVerifyingKey as PreparedVerifyingKeySoroban, Proof as ProofSoroban,
};
use soroban_sdk::{contract, contractimpl};

mod curves;
mod fields;
mod groth16;
mod helper;
mod verifier;

#[contract]
pub struct Groth16BN254Verifier;

// fn convert_g1_affine_soroban_to_ark(g1_affine_soroban: G1AffineSoroban) -> G1Affine {
//     G1Affine {
//         x: g1_affine_soroban.x,
//         y: g1_affine_soroban.y,
//         infinity: g1_affine_soroban.infinity,
//     }
// }

#[contractimpl]
impl Groth16BN254Verifier {
    // pub fn verify_proof_with_prepared_inputs(
    //     pvk: &PreparedVerifyingKey<E>,
    //     proof: &Proof<E>,
    //     prepared_inputs: &E::G1,
    // ) -> R1CSResult<bool> {

    // verify_proof_with_prepared_inputs
    pub fn verify_with_prepared_inputs(
        pvk_soroban: PreparedVerifyingKeySoroban,
        proof_soroban: ProofSoroban,
        prepared_inputs_soroban: G1ProjectiveSoroban,
    ) -> bool {
        let ark_pvk = pvk_soroban.to_ark_pvk();
        let ark_proof = proof_soroban.to_ark_proof();
        let ark_prepared_inputs = prepared_inputs_soroban.to_ark_g1_projective();

        let verify_result = Groth16::<Bn254>::verify_proof_with_prepared_inputs(
            &ark_pvk,
            &ark_proof,
            &ark_prepared_inputs,
        );

        verify_result.unwrap()
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
    }
}
