//! Primitives used in lattice-based cryptography
#![no_std]

use core::fmt::{LowerHex, UpperHex};
use subtle::{Choice, ConditionallySelectable};

/// word size of all arithmetics
pub type Word = u64;

/// An element of the prime field Z_q where q = 3329
#[derive(Debug, Clone, Copy)]
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
