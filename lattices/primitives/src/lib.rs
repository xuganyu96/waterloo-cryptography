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
    // The total number of bits must match
    assert_eq!(coeffs.len() * d, buffer.len() * (u8::BITS as usize));

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
/// Assume that the input buffer is the output of calling byte_encode with the same value for d,
/// recover the encoding back to the integer array
pub fn byte_decode(coeffs: &mut [FieldElem], d: usize, buffer: &[u8]) {
    // The total number of bits must match
    assert_eq!(coeffs.len() * d, buffer.len() * (u8::BITS as usize));

    for (byte_loc, byte) in buffer.iter().enumerate() {
        for bit_loc in 0..(u8::BITS as usize) {
            let bit = (byte >> (7 - bit_loc)) & 1;

            let coeff_loc = (byte_loc * (u8::BITS as usize) + bit_loc) / d;
            let coeff_bit_loc = (byte_loc * (u8::BITS as usize) + bit_loc) % d;
            let coeff = if bit == 1 {
                coeffs[coeff_loc].0 | (1 << coeff_bit_loc)
            } else {
                coeffs[coeff_loc].0 & (!(1 << coeff_bit_loc))
            };
            coeffs[coeff_loc] = FieldElem(coeff);
        }
    }
}

#[cfg(test)]
mod tests {
    use self::algebra::{Poly, PolyNTT};

    use super::*;
    use sha3::{
        digest::{ExtendableOutput, Update},
        Shake256,
    };

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

    #[test]
    fn encode_and_decode() {
        let mut hasher = Shake256::default();
        hasher.update(b"");
        let mut xof = hasher.finalize_xof();
        let poly = PolyNTT::sample_uniform(&mut xof).invert_ntt();

        let mut buffer = [0u8; 256 * 12 / 8];
        byte_encode(poly.as_coeffs(), 12, &mut buffer);
        let mut decoded_poly = Poly::ZERO.clone();
        byte_decode(decoded_poly.as_mut_coeffs(), 12, &buffer);

        assert_eq!(poly, decoded_poly);
    }
}
