//! Primitives used in lattice-based cryptography
#![no_std]

use core::fmt::{LowerHex, UpperHex};

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
