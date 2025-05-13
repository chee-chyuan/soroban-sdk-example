#![cfg(test)]
extern crate std;
use anyhow::anyhow;
use ark_bn254::{Bn254, G1Projective};
use ark_groth16::{Groth16, Proof};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use risc0_groth16::{ProofJson, Seal};

use super::{
    data_structures::{PublicInputsJson, VerifyingKeyJson},
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

    let pvk =
        ark_groth16::PreparedVerifyingKey::<Bn254>::deserialize_uncompressed(&mut &encoded_pvk[..])
            .map_err(|err| anyhow!(err))
            .unwrap();

    let proof = Proof::<Bn254>::deserialize_uncompressed(&mut &encoded_proof[..])
        .map_err(|err| anyhow!(err))
        .unwrap();
    let prepared_inputs =
        &G1Projective::deserialize_uncompressed(encoded_prepared_inputs.as_slice())
            .map_err(|err| anyhow!(err)).unwrap();
}

// pub fn verify(&self) -> Result<(), Error> {
//     let pvk = &PreparedVerifyingKey::deserialize_uncompressed(&*self.encoded_pvk)
//         .map_err(|err| anyhow!(err))?;
//     let proof =
//         &Proof::deserialize_uncompressed(&*self.encoded_proof).map_err(|err| anyhow!(err))?;
//     let prepared_inputs =
//         &G1Projective::deserialize_uncompressed(self.encoded_prepared_inputs.as_slice())
//             .map_err(|err| anyhow!(err))?;
//     match Groth16::<Bn254>::verify_proof_with_prepared_inputs(pvk, proof, prepared_inputs)
//         .map_err(|err| anyhow!(err))?
//     {
//         true => Ok(()),
//         false => Err(anyhow!("Invalid proof")),
//     }
// }
