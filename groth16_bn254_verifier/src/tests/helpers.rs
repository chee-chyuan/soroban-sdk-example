#![cfg(test)]

use core::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use ark_bn254::G1Affine;
use ark_bn254::G2Affine;
use ark_serialize::CanonicalDeserialize;
use num_bigint::BigInt;

extern crate std;

pub fn g1_from_bytes(elem: &[std::vec::Vec<u8>]) -> Result<G1Affine, Error> {
    if elem.len() != 2 {
        return Err(anyhow!("Malformed G1 field element"));
    }
    let g1_affine: std::vec::Vec<u8> = elem[0]
        .iter()
        .rev()
        .chain(elem[1].iter().rev())
        .cloned()
        .collect();

    G1Affine::deserialize_uncompressed(&*g1_affine).map_err(|err| anyhow!(err))
}

/// Deserialize an element over the G2 group from bytes in big-endian format
pub fn g2_from_bytes(elem: &[std::vec::Vec<std::vec::Vec<u8>>]) -> Result<G2Affine, Error> {
    if elem.len() != 2 || elem[0].len() != 2 || elem[1].len() != 2 {
        return Err(anyhow!("Malformed G2 field element"));
    }
    let g2_affine: std::vec::Vec<u8> = elem[0][1]
        .iter()
        .rev()
        .chain(elem[0][0].iter().rev())
        .chain(elem[1][1].iter().rev())
        .chain(elem[1][0].iter().rev())
        .cloned()
        .collect();

    G2Affine::deserialize_uncompressed(&*g2_affine).map_err(|err| anyhow!(err))
}

// Convert the U256 value to a byte array in big-endian format
pub fn from_u256(value: &str) -> Result<Vec<u8>, Error> {
    if let Some(stripped) = value.strip_prefix("0x") {
        from_u256_hex(stripped)
    } else {
        Ok(to_fixed_array(
            BigInt::from_str(value)
                .map_err(|_| anyhow!("conversion from u256 failed"))?
                .to_bytes_be()
                .1,
        )
        .to_vec())
    }
}

// Convert the U256 value to a byte array in big-endian format
fn from_u256_hex(value: &str) -> Result<std::vec::Vec<u8>, Error> {
    Ok(
        to_fixed_array(hex::decode(value).map_err(|_| anyhow!("conversion from u256 failed"))?)
            .to_vec(),
    )
}

fn to_fixed_array(input: Vec<u8>) -> [u8; 32] {
    let mut fixed_array = [0u8; 32];
    let start = core::cmp::max(32, input.len()) - core::cmp::min(32, input.len());
    fixed_array[start..].copy_from_slice(&input[input.len().saturating_sub(32)..]);
    fixed_array
}
