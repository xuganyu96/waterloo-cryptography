//! Primitives used in lattice-based cryptography
#![no_std]

use core::fmt::{LowerHex, UpperHex};
use sha3::digest::XofReader;
use subtle::{Choice, ConditionallySelectable};

/// word size of all arithmetics
pub type Word = u64;

/// Degree of the quotient polynomial
pub const KYBER_N: usize = 256;
pub const KYBER_Q: Word = 3329;

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
    pub const ZERO: Self = Self(0);
    pub const Q: Word = KYBER_Q;
    /// k = 2 * (floor(log2(Q)) + 1)
    pub const BARRETT_SHIFT: usize = 24;
    pub const BARRETT_MULTIPLIER: Word = (1 << Self::BARRETT_SHIFT) / KYBER_Q;

    pub fn from_u8(byte: u8) -> Self {
        Self(byte.into())
    }

    /// Assuming that the input value satisfies 0 <= val < 2*Q, perform a simple reduction
    fn simple_reduce(val: Word) -> Self {
        let val = Word::conditional_select(
            &(val.wrapping_sub(KYBER_Q)),
            &val,
            Choice::from((val < KYBER_Q) as u8),
        );

        Self(val)
    }

    /// Perform the Barrett reduction
    pub fn barrett_reduce(val: Word) -> Self {
        let quot = (val * Self::BARRETT_MULTIPLIER) >> Self::BARRETT_SHIFT;
        return Self::simple_reduce(val - quot * KYBER_Q);
    }

    /// Modulus multiplication
    pub fn modmul(&self, other: &Self) -> Self {
        Self::barrett_reduce(self.0 * other.0)
    }

    /// Modulus subtraction. Will wrap around the modulus if self is less than other
    pub fn modsub(&self, other: &Self) -> Self {
        let negwrap = Choice::from((self.0 < other.0) as u8);
        return Self::conditional_select(
            &Self(self.0.wrapping_sub(other.0)),
            &Self(KYBER_Q + self.0 - other.0),
            negwrap,
        );
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
    coeffs: [FieldElem; KYBER_N],
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
        let mut coeffs: [FieldElem; KYBER_N] = [FieldElem(0); KYBER_N];
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
    coeffs: [FieldElem; KYBER_N],
}

impl Poly {
    pub fn from_words(words: [Word; KYBER_N]) -> Self {
        let mut coeffs = [FieldElem::ZERO; KYBER_N];
        (0..KYBER_N).for_each(|i| {
            coeffs[i] = FieldElem(words[i]);
        });
        return Self { coeffs };
    }

    pub fn as_coeffs(&self) -> &[FieldElem] {
        &self.coeffs
    }

    /// Sample a polynomial from CBD(eta=2)
    pub fn sample_cbd_eta2(mut xof: impl XofReader) -> Self {
        let mut stream = [0u8; KYBER_N / 2];
        xof.read(&mut stream);
        let mut coeffs = [FieldElem::ZERO; KYBER_N];

        stream.iter().enumerate().for_each(|(i, byte)| {
            // bit operators have even lower precedence than +
            let d1 = FieldElem::from_u8(((byte >> 0) & 1) + ((byte >> 1) & 1))
                .modsub(&FieldElem::from_u8(((byte >> 2) & 1) + ((byte >> 3) & 1)));
            let d2 = FieldElem::from_u8(((byte >> 4) & 1) + ((byte >> 5) & 1))
                .modsub(&FieldElem::from_u8(((byte >> 6) & 1) + ((byte >> 7) & 1)));
            coeffs[2 * i] = d1;
            coeffs[2 * i + 1] = d2;
        });

        return Self { coeffs };
    }

    /// Sample a polynomial from CBD(eta=3)
    pub fn sample_cbd_eta3(mut xof: impl XofReader) -> Self {
        // Every 3 bytes can output 4 samples
        let mut stream = [0u8; KYBER_N / 4 * 3];
        xof.read(&mut stream);
        let mut coeffs = [FieldElem::ZERO; KYBER_N];

        // TODO: is there a way to iterate over three elements at a time?
        for i in 0..(KYBER_N / 4) {
            let b1 = stream[3 * i];
            let b2 = stream[3 * i + 1];
            let b3 = stream[3 * i + 2];

            let d1 = FieldElem::from_u8(((b1 >> 0) & 1) + ((b1 >> 1) & 1) + ((b1 >> 2) & 1))
                .modsub(&FieldElem::from_u8(
                    ((b1 >> 3) & 1) + ((b1 >> 4) & 1) + ((b1 >> 5) & 1),
                ));
            let d2 = FieldElem::from_u8(((b1 >> 6) & 1) + ((b1 >> 7) & 1) + ((b2 >> 0) & 1))
                .modsub(&FieldElem::from_u8(
                    ((b2 >> 1) & 1) + ((b2 >> 2) & 1) + ((b2 >> 3) & 1),
                ));
            let d3 = FieldElem::from_u8(((b2 >> 4) & 1) + ((b2 >> 5) & 1) + ((b2 >> 6) & 1))
                .modsub(&FieldElem::from_u8(
                    ((b2 >> 7) & 1) + ((b3 >> 0) & 1) + ((b3 >> 1) & 1),
                ));
            let d4 = FieldElem::from_u8(((b3 >> 2) & 1) + ((b3 >> 3) & 1) + ((b3 >> 4) & 1))
                .modsub(&FieldElem::from_u8(
                    ((b3 >> 5) & 1) + ((b3 >> 6) & 1) + ((b3 >> 7) & 1),
                ));
            coeffs[4 * i] = d1;
            coeffs[4 * i + 1] = d2;
            coeffs[4 * i + 2] = d3;
            coeffs[4 * i + 3] = d4;
        }

        return Self { coeffs };
    }
}

impl core::fmt::Debug for Poly {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Poly {{")?;
        for (i, coeff) in self.as_coeffs().iter().enumerate() {
            write!(f, "{}", coeff.0)?;
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
            write!(f, "0x{:03X}", coeff.0)?;
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
            write!(f, "0x{:03x}", coeff.0)?;
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
    use sha3::{
        digest::{ExtendableOutput, Update},
        Shake256,
    };

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

    #[test]
    fn field_modsub() {
        assert_eq!(FieldElem(2).modsub(&FieldElem(1)), FieldElem(1));
        assert_eq!(
            FieldElem(1).modsub(&FieldElem(2)),
            FieldElem(FieldElem::Q - 1)
        );
    }

    #[test]
    fn test_sample_cbd_eta2() {
        let mut hasher = Shake256::default();
        hasher.update(b"test seed");
        let poly = Poly::sample_cbd_eta2(hasher.finalize_xof());
        // TODO: this should be a statistical test
        assert!(poly.coeffs.contains(&FieldElem(0)));
        assert!(poly.coeffs.contains(&FieldElem(1)));
        assert!(poly.coeffs.contains(&FieldElem(2)));
        assert!(poly.coeffs.contains(&FieldElem(3328)));
        assert!(poly.coeffs.contains(&FieldElem(3327)));
    }

    #[test]
    fn test_sample_cbd_eta3() {
        let hasher = Shake256::default();
        let poly = Poly::sample_cbd_eta3(hasher.finalize_xof());
        // TODO: this should be a statistical test
        assert!(poly.coeffs.contains(&FieldElem(0)));
        assert!(poly.coeffs.contains(&FieldElem(1)));
        assert!(poly.coeffs.contains(&FieldElem(2)));
        assert!(poly.coeffs.contains(&FieldElem(3)));
        assert!(poly.coeffs.contains(&FieldElem(3328)));
        assert!(poly.coeffs.contains(&FieldElem(3327)));
        assert!(poly.coeffs.contains(&FieldElem(3326)));
    }
}
