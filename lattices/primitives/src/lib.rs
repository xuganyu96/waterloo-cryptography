//! Primitives used in lattice-based cryptography
#![no_std]

use crate::algebra::FieldElem;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128, Shake256,
};

/// word size of all arithmetics
pub type Word = u64;

pub const KYBER_N: usize = 256;
pub const KYBER_Q: Word = 3329;
pub const KYBER_Q_BITS: usize = 12;
pub const KYBER_K_512: usize = 2;
pub const KYBER_K_768: usize = 3;
pub const KYBER_K_1024: usize = 4;

/// Most seeds are 32-bytes
pub const SEEDSIZE: usize = 32;

pub mod algebra;

pub fn shake256_prf<const ETA: usize>(seed: [u8; SEEDSIZE], ctr: u8) -> [u8; ETA] {
    let mut hasher = Shake256::default();
    hasher.update(&seed);
    hasher.update(&[ctr]);
    let mut xof = hasher.finalize_xof();

    let mut output = [0u8; ETA];
    xof.read(&mut output);
    return output;
}

pub fn shake128_xof(seed: [u8; SEEDSIZE], i: u8, j: u8) -> impl XofReader {
    let mut hasher = Shake128::default();
    hasher.update(&seed);
    hasher.update(&[i, j]);
    let xof = hasher.finalize_xof();

    return xof;
}

/// Algorithm 4: ByteEncode
/// Encode an integer array into a byte array by encoding each integer using d bits
///
/// There is no easy way to define the right size of output array (and with no_std I can't use
/// Vec), so the user has to provide the buffer to write the bytes to. Fortunately, it is easy to
/// check that the input buffer has the right size
pub fn byte_encode(coeffs: &[FieldElem], d: usize, buffer: &mut [u8]) {
    // Encoding 256 integers where each integer takes d bits requires at least (32 * d) bytes
    assert!(buffer.len() == coeffs.len() * d / 8);

    for coeff_loc in 0..coeffs.len() {
        let coeff = coeffs[coeff_loc].0;
        for coeff_bit_loc in 0..d {
            let coeff_bit = (coeff >> coeff_bit_loc) & 1;

            let enc_loc = (coeff_loc * d + coeff_bit_loc) / 8;
            let enc_bit_loc = (coeff_loc * d + coeff_bit_loc) % 8;

            if coeff_bit == 1 {
                buffer[enc_loc] = buffer[enc_loc] | (0b1000_0000 >> enc_bit_loc);
            }
        }
    }
}

/// Algorithm 5: ByteDecode
pub fn byte_decode(coeffs: &mut [FieldElem], d: usize, buffer: &[u8]) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_encode_12() {
        let coeffs = [
            FieldElem(0b000000000000),
            FieldElem(0b000000000001),
            FieldElem(0b000000000010),
            FieldElem(0b000000000100),
            FieldElem(0b000000001000),
            FieldElem(0b000000010000),
            FieldElem(0b000000100000),
            FieldElem(0b000001000000),
        ];
        let mut buffer = [0u8; 12];
        byte_encode(&coeffs, 12, &mut buffer);
        assert_eq!(
            buffer,
            [
                0b0000_0000,
                0b0000_1000,
                0b0000_0000,
                0b0100_0000,
                0b0000_0010,
                0b0000_0000,
                0b0001_0000,
                0b0000_0000,
                0b1000_0000,
                0b0000_0100,
                0b0000_0000,
                0b0010_0000,
            ]
        );
    }
}
