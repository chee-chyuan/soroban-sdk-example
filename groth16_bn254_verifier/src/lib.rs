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
mod verifier;

#[contract]
pub struct Groth16BN254Verifier;

#[contractimpl]
impl Groth16BN254Verifier {
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
    }
}


mod tests;