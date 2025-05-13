#![cfg(test)]
extern crate std;
use anyhow::anyhow;
use ark_bn254::Bn254;
use ark_groth16::{Groth16, Proof};
use ark_serialize::CanonicalSerialize;
use risc0_groth16::{ProofJson, PublicInputsJson, Seal};

use super::{
    data_structures::VerifyingKeyJson,
    helpers::{g1_from_bytes, g2_from_bytes},
};

const TEST_VERIFICATION_KEY: &str = include_str!("data/verification_key.json");
const TEST_PROOF: &str = include_str!("data/proof.json");
const TEST_PUBLIC_INPUTS: &str = include_str!("data/public.json");

#[test]
fn test_verify() {
    let verifying_key: VerifyingKeyJson = serde_json::from_str(TEST_VERIFICATION_KEY).unwrap();
    let proof: ProofJson = serde_json::from_str(TEST_PROOF).unwrap();
    let public_inputs = PublicInputsJson {
        values: serde_json::from_str(TEST_PUBLIC_INPUTS).unwrap(),
    };

    let seal: Seal = proof.try_into().unwrap();
    let public_inputs_scalar = public_inputs.to_scalar().unwrap();
    let verifying_key = verifying_key.verifying_key().unwrap();

    let pvk = ark_groth16::prepare_verifying_key(&verifying_key);
    // let verifier = Verifier::new(&seal, &public_inputs_scalar, &verifying_key).unwrap();
    // verifier.verify().unwrap();

    let mut encoded_pvk = std::vec::Vec::new();
    pvk.serialize_uncompressed(&mut encoded_pvk)
        .map_err(|err| anyhow!(err))
        .unwrap();

    let mut encoded_proof = std::vec::Vec::new();
    let proof = Proof::<Bn254> {
        a: g1_from_bytes(&seal.a).unwrap(),
        b: g2_from_bytes(&seal.b).unwrap(),
        c: g1_from_bytes(&seal.c).unwrap(),
    };
    proof
        .serialize_uncompressed(&mut encoded_proof)
        .map_err(|err| anyhow!(err))
        .unwrap();

    let mut encoded_prepared_inputs = std::vec::Vec::new();

    let prepared_inputs = Groth16::<Bn254>::prepare_inputs(
        &pvk,
        &public_inputs_scalar
            .iter()
            .map(|x| x.0)
            .collect::<std::vec::Vec<_>>(),
    )
    .map_err(|err| anyhow!(err))
    .unwrap();

    prepared_inputs
        .serialize_uncompressed(&mut encoded_prepared_inputs)
        .map_err(|err| anyhow!(err))
        .unwrap();
}

// pub fn new(
//     seal: &Seal,
//     public_inputs: &[Fr],
//     verifying_key: &VerifyingKey,
// ) -> Result<Self, Error> {
//     let pvk = ark_groth16::prepare_verifying_key(&verifying_key.0);
//     let mut encoded_pvk = Vec::new();
//     pvk.serialize_uncompressed(&mut encoded_pvk)
//         .map_err(|err| anyhow!(err))?;

//     let mut encoded_proof = Vec::new();
//     let proof = Proof::<Bn254> {
//         a: g1_from_bytes(&seal.a)?,
//         b: g2_from_bytes(&seal.b)?,
//         c: g1_from_bytes(&seal.c)?,
//     };
//     proof
//         .serialize_uncompressed(&mut encoded_proof)
//         .map_err(|err| anyhow!(err))?;

//     let mut encoded_prepared_inputs = Vec::new();
//     let prepared_inputs = Groth16::<Bn254>::prepare_inputs(
//         &pvk,
//         &public_inputs.iter().map(|x| x.0).collect::<Vec<_>>(),
//     )
//     .map_err(|err| anyhow!(err))?;
//     prepared_inputs
//         .serialize_uncompressed(&mut encoded_prepared_inputs)
//         .map_err(|err| anyhow!(err))?;

//     Ok(Self {
//         encoded_pvk,
//         encoded_proof,
//         encoded_prepared_inputs,
//     })
// }

// /// Create a Verifier given the JSON representation of the proof, public inputs and verifier
// /// key.
// pub fn from_json(
//     proof: ProofJson,
//     public_inputs: PublicInputsJson,
//     verifying_key: VerifyingKeyJson,
// ) -> Result<Self> {
//     Verifier::new(
//         &proof.try_into()?,
//         &public_inputs.to_scalar()?,
//         &verifying_key.verifying_key()?,
//     )
// }
