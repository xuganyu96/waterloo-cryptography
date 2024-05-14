//! Primitives used in lattice-based cryptography
#![no_std]

use algebra::{Poly, PolyNTT};
use symmetric::shake128_xof;

use crate::{algebra::FieldElem, symmetric::hash_g};

/// word size of all arithmetics
pub type Word = u64;

pub const KYBER_N: usize = 256;
pub const KYBER_Q: Word = 3329;
pub const KYBER_Q_BITS: usize = 12;
pub const KYBER_K_512: usize = 2;
pub const KYBER_K_768: usize = 3;
pub const KYBER_K_1024: usize = 4;

/// TODO: will only implement ML_KEM_768 for now
pub const KYBER_K: usize = KYBER_K_768;
pub const KYBER_ETA_1: usize = 2;
pub const KYBER_ETA_2: usize = 2;

/// Most seeds are 32-bytes
pub const SEEDSIZE: usize = 32;

pub mod algebra;
pub mod symmetric;

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

pub struct PublicKey {
    /// The seed that generates A
    pub seed: [u8; SEEDSIZE],

    /// The k * k public matrix A
    pub a: [[PolyNTT; KYBER_K]; KYBER_K],

    /// The noisy samples: t = A * s + e
    pub t: [PolyNTT; KYBER_K],
}

impl PublicKey {
    fn sample_a(seed: [u8; SEEDSIZE]) -> [[PolyNTT; KYBER_K]; KYBER_K] {
        let mut a = [[PolyNTT::ZERO; KYBER_K]; KYBER_K];

        for i in 0..KYBER_K {
            for j in 0..KYBER_K {
                let mut xof = shake128_xof(seed, i.try_into().unwrap(), j.try_into().unwrap());
                a[i][j] = PolyNTT::sample_uniform(&mut xof);
            }
        }

        return a;
    }
}

pub struct SecretKey {
    /// The secret s
    pub sols: [PolyNTT; KYBER_K],

    /// The error e
    pub errs: [PolyNTT; KYBER_K],
}

impl SecretKey {
    fn sample_sk(seed: [u8; SEEDSIZE]) -> Self {
        let mut ctr = 0;
        let mut sols = [PolyNTT::ZERO; KYBER_K];
        let mut errs = [PolyNTT::ZERO; KYBER_K];

        for i in 0..KYBER_K {
            sols[i] = Poly::sample_cbd_eta1(&seed, ctr).ntt();
            ctr += 1;
        }

        for i in 0..KYBER_K {
            errs[i] = Poly::sample_cbd_eta1(&seed, ctr).ntt();
            ctr += 1;
        }

        return Self { sols, errs };
    }
}

pub struct KeyPair {
    pub pubkey: PublicKey,
    pub seckey: SecretKey,
}

impl KeyPair {
    pub fn keygen(seed: [u8; SEEDSIZE]) -> Self {
        let (rho, sigma) = hash_g(&seed);
        let a = PublicKey::sample_a(rho);
        let sk = SecretKey::sample_sk(sigma);
        todo!();
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
