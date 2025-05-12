// pub fn verify_proof_with_prepared_inputs(
//     pvk: &PreparedVerifyingKey<E>,
//     proof: &Proof<E>,
//     prepared_inputs: &E::G1,
// ) -> R1CSResult<bool> {
//     let qap = E::multi_miller_loop(
//         [
//             <E::G1Affine as Into<E::G1Prepared>>::into(proof.a),
//             prepared_inputs.into_affine().into(),
//             proof.c.into(),
//         ],
//         [
//             proof.b.into(),
//             pvk.gamma_g2_neg_pc.clone(),
//             pvk.delta_g2_neg_pc.clone(),
//         ],
//     );

//     let test = E::final_exponentiation(qap).ok_or(SynthesisError::UnexpectedIdentity)?;

//     Ok(test.0 == pvk.alpha_g1_beta_g2)
// }

// /// Verify a Groth16 proof `proof` against the prepared verification key `pvk`,
// /// with respect to the instance `public_inputs`.
// pub fn verify_proof(
//     pvk: &PreparedVerifyingKey<E>,
//     proof: &Proof<E>,
//     public_inputs: &[E::ScalarField],
// ) -> R1CSResult<bool> {
//     let prepared_inputs = Self::prepare_inputs(pvk, public_inputs)?;
//     Self::verify_proof_with_prepared_inputs(pvk, proof, &prepared_inputs)
// }