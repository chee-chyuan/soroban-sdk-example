#![cfg(test)]
extern crate std;
use anyhow::anyhow;
use ark_bn254::{Bn254, G1Projective};
use ark_groth16::{Groth16, Proof};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use risc0_groth16::{ProofJson, Seal};
use soroban_sdk::Env;

use crate::{
    curves::g1_projective::G1Projective as G1ProjectiveSoroban,
    groth16::data_structure::{
        PreparedVerifyingKey as SorobanPreparedVerifyingKey, Proof as SorobanProof,
    },
    Groth16BN254Verifier, Groth16BN254VerifierClient,
};

use super::{
    data_structures::{PublicInputsJson, VerifyingKeyJson},
    helpers::{g1_from_bytes, g2_from_bytes},
};

const TEST_VERIFICATION_KEY: &str = include_str!("data/verification_key.json");
const TEST_PROOF: &str = include_str!("data/proof.json");
const TEST_PUBLIC_INPUTS: &str = include_str!("data/public.json");

const TEST_VERIFICATION_KEY_MULTIPLIER: &str =
    include_str!("data_multiplier/verification_key.json");
const TEST_PROOF_MULTIPLIER: &str = include_str!("data_multiplier/proof.json");
const TEST_PUBLIC_INPUTS_MULTIPLIER: &str = include_str!("data_multiplier/public.json");

fn create_client(e: &Env) -> Groth16BN254VerifierClient {
    Groth16BN254VerifierClient::new(e, &e.register(Groth16BN254Verifier {}, ()))
}

#[test]
fn test_verify_risc0_proof() {
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
        G1Projective::deserialize_uncompressed(encoded_prepared_inputs.as_slice())
            .map_err(|err| anyhow!(err))
            .unwrap();

    // conversion
    let env = Env::default();
    let pvk_soroban = SorobanPreparedVerifyingKey::from_ark_pvk(&env, pvk);
    let proof_soroban = SorobanProof::from_ark_proof(&env, proof);
    let prepared_inputs_soroban =
        G1ProjectiveSoroban::from_ark_g1_projective(&env, prepared_inputs);

    // start test

    env.cost_estimate().budget().reset_unlimited();

    let client = create_client(&env);
    env.cost_estimate().budget().reset_default();
    let res =
        client.verify_with_prepared_inputs(&pvk_soroban, &proof_soroban, &prepared_inputs_soroban);
    assert_eq!(res, true);
    env.cost_estimate().budget().print();
}

#[test]
fn test_verify_multiplier_proof() {
    let verifying_key: VerifyingKeyJson =
        serde_json::from_str(TEST_VERIFICATION_KEY_MULTIPLIER).unwrap();
    let proof: ProofJson = serde_json::from_str(TEST_PROOF_MULTIPLIER).unwrap();
    let public_inputs = PublicInputsJson {
        values: serde_json::from_str(TEST_PUBLIC_INPUTS_MULTIPLIER).unwrap(),
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
        G1Projective::deserialize_uncompressed(encoded_prepared_inputs.as_slice())
            .map_err(|err| anyhow!(err))
            .unwrap();

    // conversion
    let env = Env::default();
    let pvk_soroban = SorobanPreparedVerifyingKey::from_ark_pvk(&env, pvk);
    let proof_soroban = SorobanProof::from_ark_proof(&env, proof);
    let prepared_inputs_soroban =
        G1ProjectiveSoroban::from_ark_g1_projective(&env, prepared_inputs);

    // start test

    env.cost_estimate().budget().reset_unlimited();

    let client = create_client(&env);
    env.cost_estimate().budget().reset_default();
    let res =
        client.verify_with_prepared_inputs(&pvk_soroban, &proof_soroban, &prepared_inputs_soroban);
    assert_eq!(res, true);
    env.cost_estimate().budget().print();
}






// stellar contract upload \
//   --network testnet \
//   --source SBQZIRY4U6HEWA7INGHM2ZJIUZ2NUOUEAKEQQKV67GYD4S2WWAW66GDY \
//   --wasm target/wasm32-unknown-unknown/release/soroban_groth16_bn254_verifier_contract.wasm 

// wasm hash
// 0a607c9efe9022fb5088a56b88127a50e5bdc6acd34408d1dfa18bcf7c9a8a51

// stellar contract deploy \
//   --wasm-hash 0a607c9efe9022fb5088a56b88127a50e5bdc6acd34408d1dfa18bcf7c9a8a51 \
//   --source SBQZIRY4U6HEWA7INGHM2ZJIUZ2NUOUEAKEQQKV67GYD4S2WWAW66GDY \
//   --network testnet \
//   --alias groth16_verifier_bn254

// contract id
// https://stellar.expert/explorer/testnet/contract/CD2MSG6X4FHAA5UKE2QJLEM53IMPYGWCK2C2KHNEPUB6GFUXPZ4M345K
// CD2MSG6X4FHAA5UKE2QJLEM53IMPYGWCK2C2KHNEPUB6GFUXPZ4M345K


// stellar contract bindings typescript \
//   --network testnet \
//   --contract-id hello_world \
//   --output-dir packages/hello_world