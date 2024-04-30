//! Primitives used in lattice-based cryptography
#![no_std]

use core::fmt::{LowerHex, UpperHex};
use sha3::digest::XofReader;
use subtle::{Choice, ConditionallySelectable};

/// word size of all arithmetics
pub type Word = u64;

/// Degree of the quotient polynomial
pub const N: usize = 256;

/// An element of the prime field Z_q where q = 3329
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElem(pub Word);

impl ConditionallySelectable for FieldElem {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let val = u64::conditional_select(&a.0, &b.0, choice);
        return Self(val);
    }
}

impl FieldElem {
    pub const Q: Word = 3329;
    /// k = 2 * (floor(log2(Q)) + 1)
    pub const BARRETT_SHIFT: usize = 24;
    pub const BARRETT_MULTIPLIER: Word = (1 << Self::BARRETT_SHIFT) / Self::Q;

    /// Assuming that the input value satisfies 0 <= val < 2*Q, perform a simple reduction
    fn simple_reduce(val: Word) -> Self {
        let val = Word::conditional_select(
            &(val.wrapping_sub(Self::Q)),
            &val,
            Choice::from((val < Self::Q) as u8),
        );

        Self(val)
    }

    /// Perform the Barrett reduction
    pub fn barrett_reduce(val: Word) -> Self {
        let quot = (val * Self::BARRETT_MULTIPLIER) >> Self::BARRETT_SHIFT;
        return Self::simple_reduce(val - quot * Self::Q);
    }

    pub fn modmul(&self, other: &Self) -> Self {
        Self::barrett_reduce(self.0 * other.0)
    }
}

/// A member of the polynomial ring R_q in NTT domain
///
/// Mathematically, the NTT representation of some polynomial in R_q consists of 128 polynomials
/// each of degree 1, but we don't impose additional structure on the coefficients so the sampling
/// and transformation methods can be easier to implement
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PolyNTT {
    /// Entries of the NTT representation
    coeffs: [FieldElem; N],
}

impl PolyNTT {
    /// Algorithm 6: Sample from uniform distribution
    ///
    /// Since q = 3329 takes 12 bits, we try to sample 2 entries from 3 bytes at a time. Each entry
    /// is sampled from a random 12 bit string, but if the 12-bit integer is too large, it is
    /// rejected. Conditioned on the 12-bit integer being less than Q, the sampled coefficient
    /// follows a uniform distribution within Z_q
    pub fn sample(mut xof: impl XofReader) -> Self {
        let mut j: usize = 0;
        let mut coeffs: [FieldElem; N] = [FieldElem(0); N];
        let mut stream = [0u8; 3];

        while j < 256 {
            xof.read(&mut stream);
            let b1: Word = stream[0].into();
            let b2: Word = stream[1].into();
            let b3: Word = stream[2].into();
            let d1 = b1 + (b2 & 0x0F) << 8;
            let d2 = b2 >> 4 + b3 << 4;
            if d1 < FieldElem::Q {
                coeffs[j] = FieldElem(d1);
                j += 1;
            }
            if d2 < FieldElem::Q && j < 256 {
                coeffs[j] = FieldElem(d2);
                j += 1;
            }
        }

        return Self { coeffs };
    }
}

/// A member of the polynomial ring used in Kyber:
/// R_q = Z_q[x] / (x ** n + 1)
/// where q is 3329 = 2 ** 8 * 13 + 1 and n is 256
#[derive(Copy, Clone, PartialEq)]
pub struct Poly {
    /// Each member is uniquely determined by its coefficients
    /// Coefficient at lower index correspond to term with lower power
    ///
    /// NOTE: something about using a larger uint type than strictly necessary (3329 < 2 ** 16)
    /// because it's easier to do modular arithmetic with
    coeffs: [u32; 256],
}

impl Poly {
    pub fn from_coeffs(coeffs: [u32; 256]) -> Self {
        Self { coeffs }
    }

    pub fn as_coeffs(&self) -> &[u32] {
        &self.coeffs
    }
}

impl core::fmt::Debug for Poly {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Poly {{")?;
        for (i, coeff) in self.as_coeffs().iter().enumerate() {
            write!(f, "{}", coeff)?;
            if i < self.as_coeffs().len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;

        return Ok(());
    }
}

impl core::fmt::Display for Poly {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}

// Hexadecimal representation is still meaningful since we will be dealing with compression and
// decompression later on
impl UpperHex for Poly {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Poly {{")?;
        for (i, coeff) in self.as_coeffs().iter().enumerate() {
            // 3329 < 2 ** 12, so 12 bits is sufficient for representation
            write!(f, "0x{:03X}", coeff)?;
            if i < self.as_coeffs().len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;

        return Ok(());
    }
}

impl LowerHex for Poly {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Poly {{")?;
        for (i, coeff) in self.as_coeffs().iter().enumerate() {
            // 3329 < 2 ** 12, so 12 bits is sufficient for representation
            write!(f, "0x{:03x}", coeff)?;
            if i < self.as_coeffs().len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_modmul() {
        for a in 0..FieldElem::Q {
            for b in 0..FieldElem::Q {
                let barret_modmul = FieldElem(a).modmul(&FieldElem(b)).0;
                let expected = (a * b) % FieldElem::Q;
                assert_eq!(barret_modmul, expected);
            }
        }
    }
}
