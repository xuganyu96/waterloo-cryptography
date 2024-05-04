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

/// zeta (the 256-th primitive root of 3329) and its powers, up to 127
pub const ZETA_POWS: [FieldElem; 256] = [
    FieldElem(1),
    FieldElem(17),
    FieldElem(289),
    FieldElem(1584),
    FieldElem(296),
    FieldElem(1703),
    FieldElem(2319),
    FieldElem(2804),
    FieldElem(1062),
    FieldElem(1409),
    FieldElem(650),
    FieldElem(1063),
    FieldElem(1426),
    FieldElem(939),
    FieldElem(2647),
    FieldElem(1722),
    FieldElem(2642),
    FieldElem(1637),
    FieldElem(1197),
    FieldElem(375),
    FieldElem(3046),
    FieldElem(1847),
    FieldElem(1438),
    FieldElem(1143),
    FieldElem(2786),
    FieldElem(756),
    FieldElem(2865),
    FieldElem(2099),
    FieldElem(2393),
    FieldElem(733),
    FieldElem(2474),
    FieldElem(2110),
    FieldElem(2580),
    FieldElem(583),
    FieldElem(3253),
    FieldElem(2037),
    FieldElem(1339),
    FieldElem(2789),
    FieldElem(807),
    FieldElem(403),
    FieldElem(193),
    FieldElem(3281),
    FieldElem(2513),
    FieldElem(2773),
    FieldElem(535),
    FieldElem(2437),
    FieldElem(1481),
    FieldElem(1874),
    FieldElem(1897),
    FieldElem(2288),
    FieldElem(2277),
    FieldElem(2090),
    FieldElem(2240),
    FieldElem(1461),
    FieldElem(1534),
    FieldElem(2775),
    FieldElem(569),
    FieldElem(3015),
    FieldElem(1320),
    FieldElem(2466),
    FieldElem(1974),
    FieldElem(268),
    FieldElem(1227),
    FieldElem(885),
    FieldElem(1729),
    FieldElem(2761),
    FieldElem(331),
    FieldElem(2298),
    FieldElem(2447),
    FieldElem(1651),
    FieldElem(1435),
    FieldElem(1092),
    FieldElem(1919),
    FieldElem(2662),
    FieldElem(1977),
    FieldElem(319),
    FieldElem(2094),
    FieldElem(2308),
    FieldElem(2617),
    FieldElem(1212),
    FieldElem(630),
    FieldElem(723),
    FieldElem(2304),
    FieldElem(2549),
    FieldElem(56),
    FieldElem(952),
    FieldElem(2868),
    FieldElem(2150),
    FieldElem(3260),
    FieldElem(2156),
    FieldElem(33),
    FieldElem(561),
    FieldElem(2879),
    FieldElem(2337),
    FieldElem(3110),
    FieldElem(2935),
    FieldElem(3289),
    FieldElem(2649),
    FieldElem(1756),
    FieldElem(3220),
    FieldElem(1476),
    FieldElem(1789),
    FieldElem(452),
    FieldElem(1026),
    FieldElem(797),
    FieldElem(233),
    FieldElem(632),
    FieldElem(757),
    FieldElem(2882),
    FieldElem(2388),
    FieldElem(648),
    FieldElem(1029),
    FieldElem(848),
    FieldElem(1100),
    FieldElem(2055),
    FieldElem(1645),
    FieldElem(1333),
    FieldElem(2687),
    FieldElem(2402),
    FieldElem(886),
    FieldElem(1746),
    FieldElem(3050),
    FieldElem(1915),
    FieldElem(2594),
    FieldElem(821),
    FieldElem(641),
    FieldElem(910),
    FieldElem(2154),
    FieldElem(3328),
    FieldElem(3312),
    FieldElem(3040),
    FieldElem(1745),
    FieldElem(3033),
    FieldElem(1626),
    FieldElem(1010),
    FieldElem(525),
    FieldElem(2267),
    FieldElem(1920),
    FieldElem(2679),
    FieldElem(2266),
    FieldElem(1903),
    FieldElem(2390),
    FieldElem(682),
    FieldElem(1607),
    FieldElem(687),
    FieldElem(1692),
    FieldElem(2132),
    FieldElem(2954),
    FieldElem(283),
    FieldElem(1482),
    FieldElem(1891),
    FieldElem(2186),
    FieldElem(543),
    FieldElem(2573),
    FieldElem(464),
    FieldElem(1230),
    FieldElem(936),
    FieldElem(2596),
    FieldElem(855),
    FieldElem(1219),
    FieldElem(749),
    FieldElem(2746),
    FieldElem(76),
    FieldElem(1292),
    FieldElem(1990),
    FieldElem(540),
    FieldElem(2522),
    FieldElem(2926),
    FieldElem(3136),
    FieldElem(48),
    FieldElem(816),
    FieldElem(556),
    FieldElem(2794),
    FieldElem(892),
    FieldElem(1848),
    FieldElem(1455),
    FieldElem(1432),
    FieldElem(1041),
    FieldElem(1052),
    FieldElem(1239),
    FieldElem(1089),
    FieldElem(1868),
    FieldElem(1795),
    FieldElem(554),
    FieldElem(2760),
    FieldElem(314),
    FieldElem(2009),
    FieldElem(863),
    FieldElem(1355),
    FieldElem(3061),
    FieldElem(2102),
    FieldElem(2444),
    FieldElem(1600),
    FieldElem(568),
    FieldElem(2998),
    FieldElem(1031),
    FieldElem(882),
    FieldElem(1678),
    FieldElem(1894),
    FieldElem(2237),
    FieldElem(1410),
    FieldElem(667),
    FieldElem(1352),
    FieldElem(3010),
    FieldElem(1235),
    FieldElem(1021),
    FieldElem(712),
    FieldElem(2117),
    FieldElem(2699),
    FieldElem(2606),
    FieldElem(1025),
    FieldElem(780),
    FieldElem(3273),
    FieldElem(2377),
    FieldElem(461),
    FieldElem(1179),
    FieldElem(69),
    FieldElem(1173),
    FieldElem(3296),
    FieldElem(2768),
    FieldElem(450),
    FieldElem(992),
    FieldElem(219),
    FieldElem(394),
    FieldElem(40),
    FieldElem(680),
    FieldElem(1573),
    FieldElem(109),
    FieldElem(1853),
    FieldElem(1540),
    FieldElem(2877),
    FieldElem(2303),
    FieldElem(2532),
    FieldElem(3096),
    FieldElem(2697),
    FieldElem(2572),
    FieldElem(447),
    FieldElem(941),
    FieldElem(2681),
    FieldElem(2300),
    FieldElem(2481),
    FieldElem(2229),
    FieldElem(1274),
    FieldElem(1684),
    FieldElem(1996),
    FieldElem(642),
    FieldElem(927),
    FieldElem(2443),
    FieldElem(1583),
    FieldElem(279),
    FieldElem(1414),
    FieldElem(735),
    FieldElem(2508),
    FieldElem(2688),
    FieldElem(2419),
    FieldElem(1175),
];

/// Reverse the bits of a 7-bit unsigned integer.
///
/// It is asssumed that 0 <= val < 128
const fn bitrev7(val: u8) -> u8 {
    // NOTE: cursed bit arithmetic. Hopefully it is evaluated at compile time!
    let mut bitrev = 0;
    let mut scan = 0;
    while scan <= 7 {
        if (val & (1u8 << scan)) != 0 {
            bitrev |= 0b0100_0000 >> scan;
        }
        scan += 1;
    }

    return bitrev;
}

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

    /// Modulus addition
    pub fn modadd(&self, other: &Self) -> Self {
        // Direct addition is guaranteed to be at most 1 modulus reduction away correct
        Self::simple_reduce(self.0 + other.0)
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
            let d1 = b1 + ((b2 & 0x0F) << 8);
            let d2 = (b2 >> 4) + (b3 << 4);
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

    /// Algorithm 9: Invert the NTT transformation
    ///
    /// This is an in-place transformation so self is consumed
    pub fn invert_ntt(self) -> Poly {
        let mut coeffs = self.coeffs;
        let mut k = 127;
        let mut len = 2;
        while len <= 128 {
            let mut start = 0;
            while start < 256 {
                let zeta = ZETA_POWS[bitrev7(k) as usize];
                k -= 1;
                let mut j = start;
                while j < start + len {
                    let t = coeffs[j];
                    coeffs[j] = t.modadd(&coeffs[j + len]);
                    coeffs[j + len] = zeta.modmul(&coeffs[j + len].modsub(&t));

                    j += 1;
                }

                start += 2 * len;
            }

            len = len * 2;
        }
        for i in 0..KYBER_N {
            coeffs[i] = coeffs[i].modmul(&FieldElem(3303));
        }

        return Poly { coeffs };
    }

    /// Algorithm 11: BaseCaseMultiply
    ///
    /// Multiply two degree-one polynomial modulus (x^2 - gamma)
    fn base_polymul(
        a0: &FieldElem,
        a1: &FieldElem,
        b0: &FieldElem,
        b1: &FieldElem,
        gamma: &FieldElem,
    ) -> (FieldElem, FieldElem) {
        return (
            a0.modmul(b0).modadd(&a1.modmul(b1).modmul(gamma)),
            a0.modmul(b1).modadd(&a1.modmul(b0)),
        );
    }

    /// Algorithm 10: MultiplyNTTs
    /// Multiply two polynomials in the NTT domain
    pub fn polymul(&self, other: &PolyNTT) -> PolyNTT {
        let mut coeffs = [FieldElem::ZERO; KYBER_N];

        for i in 0..128 {
            let (h0, h1) = Self::base_polymul(
                &self.coeffs[2 * i],
                &self.coeffs[2 * i + 1],
                &other.coeffs[2 * i],
                &other.coeffs[2 * i + 1],
                &ZETA_POWS[bitrev7(i as u8) as usize * 2 + 1],
            );
            coeffs[2 * i] = h0;
            coeffs[2 * i + 1] = h1;
        }

        return PolyNTT { coeffs };
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

    /// Algorithm 8: NTT transformation
    /// This is an in-place transformation, so the input will be consumed
    pub fn ntt(self) -> PolyNTT {
        let mut coeffs = self.coeffs;

        let mut k = 1;
        let mut len = 128;
        while len >= 2 {
            let mut start = 0;
            while start < 256 {
                let zeta = ZETA_POWS[bitrev7(k) as usize];
                k += 1;
                let mut j = start;
                while j < start + len {
                    let t = zeta.modmul(&coeffs[j + len]);
                    coeffs[j + len] = coeffs[j].modsub(&t);
                    coeffs[j] = coeffs[j].modadd(&t);
                    j += 1;
                }

                start += 2 * len;
            }

            len = len >> 1;
        }

        return PolyNTT { coeffs };
    }

    /// Polynomial multiplication using convolution
    pub fn polymul(&self, other: &Self) -> Self {
        let mut coeffs = [FieldElem::ZERO; KYBER_N];

        for self_exp in 0..KYBER_N {
            for other_exp in 0..KYBER_N {
                let mut coeff = self.coeffs[self_exp].modmul(&other.coeffs[other_exp]);
                let prod_exp = if self_exp + other_exp >= KYBER_N {
                    coeff = coeff.modmul(&FieldElem(KYBER_Q - 1));
                    self_exp + other_exp - KYBER_N
                } else {
                    self_exp + other_exp
                };
                coeffs[prod_exp] = coeffs[prod_exp].modadd(&coeff);
            }
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
    fn field_modadd() {
        for a in 0..FieldElem::Q {
            for b in 0..FieldElem::Q {
                let result = FieldElem(a).modadd(&FieldElem(b)).0;
                let expected = (a + b) % FieldElem::Q;
                assert_eq!(result, expected);
            }
        }
    }

    #[test]
    fn field_modsub() {
        for a in 0..FieldElem::Q {
            for b in 0..FieldElem::Q {
                let result = FieldElem(a).modsub(&FieldElem(b)).0;
                let expected = if a >= b {
                    (a - b) % FieldElem::Q
                } else {
                    (a + FieldElem::Q - b) % FieldElem::Q
                };
                assert_eq!(result, expected);
            }
        }
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

    #[test]
    fn test_bitrev7() {
        assert_eq!(bitrev7(0), 0);
        assert_eq!(bitrev7(0b111_1111), 0b111_1111);
        assert_eq!(bitrev7(0b111_0000), 0b000_0111);
        assert_eq!(bitrev7(0b100_0000), 0b000_0001);
    }

    /// Do a NTT on a random polynomial and invert. Check that the inversion is correct
    #[test]
    fn ntt_and_invert() {
        let mut hasher = Shake256::default();
        hasher.update(b"test seed");
        let poly = Poly::sample_cbd_eta2(hasher.finalize_xof());
        let ntt = poly.clone().ntt();
        let inverse = ntt.invert_ntt();
        assert_eq!(inverse, poly);
    }

    /// Sanity check for base case polynomial multiplication
    #[test]
    fn base_polymul() {
        assert_eq!(
            PolyNTT::base_polymul(
                &FieldElem(0),
                &FieldElem(0),
                &FieldElem(0),
                &FieldElem(0),
                &ZETA_POWS[1],
            ),
            (FieldElem(0), FieldElem(0))
        );
        assert_eq!(
            PolyNTT::base_polymul(
                &FieldElem(3326),
                &FieldElem(1234),
                &FieldElem(876),
                &FieldElem(543),
                &ZETA_POWS[1],
            ),
            (FieldElem(3246), FieldElem(759))
        );
    }

    /// Sanity check for polynomial multiplication in NTT
    #[test]
    fn polymul_ntt() {
        let mut hasher = Shake256::default();
        hasher.update(b"test poly 1");
        let poly1_ntt = PolyNTT::sample(hasher.finalize_xof());
        let mut hasher = Shake256::default();
        hasher.update(b"test poly 2");
        let poly2_ntt = PolyNTT::sample(hasher.finalize_xof());

        let ntt_prod = poly1_ntt.polymul(&poly2_ntt).invert_ntt();

        let poly1 = poly1_ntt.invert_ntt();
        let poly2 = poly2_ntt.invert_ntt();
        let conv_prod = poly1.polymul(&poly2);

        assert_eq!(ntt_prod, conv_prod);
    }
}
