#![cfg(test)]

use anyhow::{anyhow, Error};
use ark_bn254::Bn254;
use core::str::FromStr;
use serde::{Deserialize, Serialize};

use super::helpers::{from_u256, g1_from_bytes, g2_from_bytes};
extern crate std;

/// Groth16 Verifying Key encoded as JSON.
#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyingKeyJson {
    protocol: std::string::String,
    curve: std::string::String,
    #[serde(rename = "nPublic")]
    n_public: u32,
    vk_alpha_1: std::vec::Vec<std::string::String>,
    vk_beta_2: std::vec::Vec<std::vec::Vec<std::string::String>>,
    vk_gamma_2: std::vec::Vec<std::vec::Vec<std::string::String>>,
    vk_delta_2: std::vec::Vec<std::vec::Vec<std::string::String>>,
    vk_alphabeta_12: std::vec::Vec<std::vec::Vec<std::vec::Vec<std::string::String>>>,
    #[serde(rename = "IC")]
    ic: std::vec::Vec<std::vec::Vec<std::string::String>>,
}

impl VerifyingKeyJson {
    /// Computes the prepared verifying key
    pub fn verifying_key(&self) -> Result<ark_groth16::VerifyingKey<Bn254>, Error> {
        if self.vk_alpha_1.len() < 2 {
            return Err(anyhow!("Malformed G1 element field: vk_alpha_1"));
        }
        let alpha_g1 = g1_from_bytes(&[
            from_u256(&self.vk_alpha_1[0])?,
            from_u256(&self.vk_alpha_1[1])?,
        ])?;

        if self.vk_beta_2.len() < 2 || self.vk_beta_2[0].len() < 2 || self.vk_beta_2[1].len() < 2 {
            return Err(anyhow!("Malformed G2 element field: vk_beta_2"));
        }
        let beta_g2 = g2_from_bytes(&[
            std::vec![
                from_u256(&self.vk_beta_2[0][1])?,
                from_u256(&self.vk_beta_2[0][0])?,
            ],
            std::vec![
                from_u256(&self.vk_beta_2[1][1])?,
                from_u256(&self.vk_beta_2[1][0])?,
            ],
        ])?;

        if self.vk_gamma_2.len() < 2 || self.vk_gamma_2[0].len() < 2 || self.vk_gamma_2[1].len() < 2
        {
            return Err(anyhow!("Malformed G2 element field: vk_gamma_2"));
        }
        let gamma_g2 = g2_from_bytes(&[
            std::vec![
                from_u256(&self.vk_gamma_2[0][1])?,
                from_u256(&self.vk_gamma_2[0][0])?,
            ],
            std::vec![
                from_u256(&self.vk_gamma_2[1][1])?,
                from_u256(&self.vk_gamma_2[1][0])?,
            ],
        ])?;

        if self.vk_delta_2.len() < 2 || self.vk_delta_2[0].len() < 2 || self.vk_delta_2[1].len() < 2
        {
            return Err(anyhow!("Malformed G2 element field: vk_delta_2"));
        }
        let delta_g2 = g2_from_bytes(&[
            std::vec![
                from_u256(&self.vk_delta_2[0][1])?,
                from_u256(&self.vk_delta_2[0][0])?,
            ],
            std::vec![
                from_u256(&self.vk_delta_2[1][1])?,
                from_u256(&self.vk_delta_2[1][0])?,
            ],
        ])?;

        let gamma_abc_g1 = self
            .ic
            .iter()
            .enumerate()
            .map(|(i, ic)| {
                if ic.len() < 2 {
                    return Err(anyhow!("Malformed G1 element field: IC_{i}"));
                }
                g1_from_bytes(&[from_u256(&ic[0])?, from_u256(&ic[1])?])
            })
            .collect::<Result<std::vec::Vec<_>, _>>()?;

        Ok(ark_groth16::VerifyingKey::<Bn254> {
            alpha_g1,
            beta_g2,
            gamma_g2,
            delta_g2,
            gamma_abc_g1,
        })
    }
}

/// Compatibility module providing simple serde interop
mod serde_ark {
    extern crate alloc;
    use alloc::vec::Vec;

    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use serde::{
        de::Error as _, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer,
    };

    pub fn serialize<S>(key: &impl CanonicalSerialize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buffer = Vec::<u8>::new();
        key.serialize_uncompressed(&mut buffer)
            .map_err(S::Error::custom)?;
        buffer.serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: CanonicalDeserialize,
    {
        let buffer = Vec::<u8>::deserialize(deserializer)?;
        T::deserialize_uncompressed(buffer.as_slice()).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fr(#[serde(with = "serde_ark")] pub ark_bn254::Fr);

impl Fr {
    pub fn ark_fr(&self) -> ark_bn254::Fr {
        self.0
    }
}

// impl Digestible for Fr {
//     /// Compute a tagged hash of the [Fr] value.
//     fn digest<S: Sha256>(&self) -> Digest {
//         let mut buffer = Vec::<u8>::with_capacity(32);
//         // Serialization into a pre-allocated buffer should never fail.
//         self.0.serialize_uncompressed(&mut buffer).unwrap();
//         // Convert to big-endian representation.
//         buffer.reverse();
//         tagged_struct::<S>(
//             "risc0_groth16.Fr",
//             &[bytemuck::pod_read_unaligned::<Digest>(&buffer)],
//             &[],
//         )
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicInputsJson {
    /// values of the public witness
    pub values: std::vec::Vec<std::string::String>,
}

impl PublicInputsJson {
    /// Converts public inputs to scalars over the field of the G1/G2 groups.
    pub fn to_scalar(&self) -> Result<std::vec::Vec<Fr>, Error> {
        self.values
            .iter()
            .map(|input| {
                ark_bn254::Fr::from_str(input)
                    .map(Fr)
                    .map_err(|_| anyhow!("Failed to decode 'public inputs' values"))
            })
            .collect()
    }
}
