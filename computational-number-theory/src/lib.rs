//! My implementation of computational number theory
//! Follows Henri Cohen's [A Course in Computational Algebraic Number Theory]

use std::{cmp::Ordering, fmt::UpperHex, num::ParseIntError};

/// 32-bit architecture
pub type Word = u32;
/// Twice the size of Word, useful for widening multiplication
pub type LongWord = u64;

/// Equivalent to carrying_add but I have to implement it because carrying_add is not stable
const fn overflowing_adc(a: Word, b: Word, carry: bool) -> (Word, bool) {
    // There are two overflowing addition to perform
    let (mut sum, mut carry_next) = a.overflowing_add(b);
    if carry {
        let (carried_sum, carry_2) = sum.overflowing_add(1);
        carry_next = carry_next || carry_2;
        sum = carried_sum;
    }

    return (sum, carry_next);
}

/// Equivalent to borrowing_sub but borrowing_sub is unstable
const fn overflowing_sbb(a: Word, b: Word, borrow: bool) -> (Word, bool) {
    let (mut diff, mut borrow_next) = a.overflowing_sub(b);
    if borrow {
        let (borrowed_diff, borrow_2) = diff.overflowing_sub(1);
        borrow_next = borrow_next || borrow_2;
        diff = borrowed_diff;
    }

    return (diff, borrow_next);
}

fn widening_mul(a: Word, b: Word) -> (Word, Word) {
    let a: LongWord = a.into();
    let b: LongWord = b.into();
    let prod: LongWord = a.wrapping_mul(b);
    let low: Word = prod as Word;
    let high: Word = (prod >> 32) as Word;

    return (high, low);
}

/// Count the number of leading zero bits
/// TODO: there is probably more efficient implementation but oh well
const fn leading_zeros(word: Word) -> u32 {
    let mut cnt = 0;
    let mask = 1 << 31;
    let mut shift = 0;

    while shift < 32 {
        if word & (mask >> shift) == 0 {
            cnt += 1;
        } else {
            return cnt;
        }

        shift += 1;
    }

    return cnt;
}

/// A big integer is an array of words
/// We will tentatively use little-endian: lower index encodes less significant digits
/// num = sum_{i=0}^{L-1}(Uint[i] * (Word ** i))
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Uint<const L: usize>([Word; L]);

impl<const L: usize> Uint<L> {
    pub const ZERO: Self = Self([0; L]);
    pub const MAX: Self = Self([Word::MAX; L]);
    pub const ONE: Self = {
        let mut arr = [0; L];
        arr[0] = 1;
        Self(arr)
    };
    pub const BITS: u32 = 32 * (L as u32);
    pub const BYTES: u32 = 4 * (L as u32);

    /// Parse from hexadecimal string to big integer. Will panic if the string is invalid, such as
    /// incorrect length or invalid encoding
    pub fn from_be_hex(hexstr: &str) -> Result<Self, ParseIntError> {
        assert_eq!(hexstr.len(), (Self::BYTES as usize) * 2);

        let mut val = Self::ZERO;
        for i in 0..L {
            let start = (L - 1 - i) * 8;
            let stop = (L - i) * 8;
            val.0[i] = Word::from_str_radix(&hexstr[start..stop], 16)?;
        }

        return Ok(val);
    }

    /// Add "word * (base ** pow)" to self in-place. Return True iff the sum overflows the
    /// representable range by Self
    fn add_word_inplace(&mut self, word: Word, pow: usize) -> bool {
        assert!(
            pow < L,
            "Attempted to add unrepresentable power {} to Uint<{}>",
            pow,
            L
        );
        let (sum, carry) = self.0[pow].overflowing_add(word);
        self.0[pow] = sum;

        if carry {
            if pow == L - 1 {
                return true;
            } else {
                // TODO: probably want to get rid of this recursive call
                return self.add_word_inplace(1, pow + 1);
            }
        }

        return false;
    }

    /// Return (sum, carry) where carry is true if and only if the true sum overflows the capacity
    /// of the big integer
    pub const fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        let mut sum = Self::ZERO;
        let mut carry = false;
        let mut i = 0;

        while i < L {
            let (word_sum, carry_next) = overflowing_adc(self.0[i], other.0[i], carry);
            sum.0[i] = word_sum;
            carry = carry_next;
            i += 1;
        }

        return (sum, carry);
    }

    /// Return (diff, borrow) where borrow is true if self is less than other
    pub const fn overflowing_sub(&self, other: &Self) -> (Self, bool) {
        let mut diff = Self::ZERO;
        let mut borrow = false;
        let mut i = 0;

        while i < L {
            let (word_diff, borrow_next) = overflowing_sbb(self.0[i], other.0[i], borrow);
            diff.0[i] = word_diff;
            borrow = borrow_next;
            i += 1;
        }

        return (diff, borrow);
    }

    /// Schoolbook multiplication O(n^2), return (high, low)
    pub fn widening_mul(&self, other: &Self) -> (Self, Self) {
        let (mut high, mut low) = (Uint([0; L]), Uint([0; L]));
        let mut self_loc = 0;

        while self_loc < L {
            let self_val = self.0[self_loc];

            let mut other_loc = 0;
            while other_loc < L {
                let other_val = other.0[other_loc];
                let (tmp_high, tmp_low) = widening_mul(self_val, other_val);
                let tmp_high_loc = self_loc + other_loc + 1;
                let tmp_low_loc = self_loc + other_loc;

                if tmp_high_loc >= L {
                    high.add_word_inplace(tmp_high, tmp_high_loc - L);
                } else {
                    let carry = low.add_word_inplace(tmp_high, tmp_high_loc);
                    if carry {
                        high.add_word_inplace(1, 0);
                    }
                }

                if tmp_low_loc >= L {
                    high.add_word_inplace(tmp_low, tmp_low_loc - L);
                } else {
                    let carry = low.add_word_inplace(tmp_low, tmp_low_loc);
                    if carry {
                        high.add_word_inplace(1, 0);
                    }
                }

                other_loc += 1;
            }
            self_loc += 1;
        }

        return (high, low);
    }

    pub const fn overflowing_shl(&self, shift: usize) -> (Self, bool) {
        let mut shifted = Self::ZERO;

        let word_shift = shift / (Word::BITS as usize);
        let overflow = shift % (Word::BITS as usize);

        let mut i = 0;
        while i < L {
            if i + word_shift < L {
                shifted.0[i + word_shift] |= self.0[i] << overflow;
            }
            if i + word_shift + 1 < L && (overflow != 0) {
                shifted.0[i + word_shift + 1] |= self.0[i] >> (Word::BITS as usize - overflow);
            }

            i += 1;
        }

        return (shifted, shift >= (Self::BITS as usize));
    }

    pub fn overflowing_shr(&self, shift: usize) -> (Self, bool) {
        let mut shifted = Self::ZERO;

        let word_shift = shift / (Word::BITS as usize);
        let overflow = shift % (Word::BITS as usize);

        let mut i = L - 1;
        loop {
            if i >= word_shift {
                shifted.0[i - word_shift] |= self.0[i] >> overflow;
            }
            if i >= word_shift + 1 && (overflow != 0) {
                shifted.0[i - word_shift - 1] |= self.0[i] << (Word::BITS as usize - overflow);
            }

            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        return (shifted, shift >= (Self::BITS as usize));
    }

    /// Comparison, but running through all digits
    pub fn cmp(&self, other: &Self) -> Ordering {
        let mut i = L - 1;
        let mut ordering = Ordering::Equal;
        loop {
            if ordering == Ordering::Equal {
                ordering = self.0[i].cmp(&other.0[i]);
            }

            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        return ordering;
    }

    /// The number of leading zero bits
    pub const fn leading_zeros(&self) -> u32 {
        let mut cnt = 0;
        let mut i = 0;

        while i < L {
            let word = self.0[L - i - 1];
            let word_leading_zeros = leading_zeros(word);
            cnt += word_leading_zeros;
            if word != 0 {
                return cnt;
            }
            i += 1;
        }

        return cnt;
    }

    /// Euclidean division (self / other), returning the quotient and the remainder such that
    /// `self = other * quotient + remainder`
    /// There is no constant-time guarantee of any sort
    /// It is probably also very inefficient
    pub fn div_rem_vartime(&self, other: &Self) -> (Self, Self) {
        let mut quo = Self::ZERO;
        let mut rem = self.clone();
        // TODO: this is the source of timing variability
        let leading_zeros: usize = other.leading_zeros() as usize;
        let mut i = 0;

        while i <= leading_zeros {
            let other_shift = leading_zeros - i;
            let (shifted, _) = other.overflowing_shl(other_shift);
            let (multiplier, _) = Self::ONE.overflowing_shl(other_shift);
            if rem >= shifted {
                let (diff, _) = rem.overflowing_sub(&shifted);
                let (sum, _) = quo.overflowing_add(&multiplier);
                rem = diff;
                quo = sum;
            }

            i += 1;
        }

        return (quo, rem);
    }
}

pub type U256 = Uint<8>;

impl<const L: usize> UpperHex for Uint<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..L {
            write!(f, "{:08X}", self.0[L - i - 1])?;
        }
        return Ok(());
    }
}

impl<const L: usize> PartialOrd for Uint<L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_be_hex() {
        assert_eq!(
            U256::from_be_hex("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap(),
            U256::ZERO,
        );

        assert_eq!(
            U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
                .unwrap(),
            U256::MAX,
        );

        assert_eq!(
            U256::from_be_hex("3333333322222222111111110000000076543210FEDCBA9889ABCDEF01234567")
                .unwrap(),
            Uint([
                0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210, 0x00000000, 0x11111111, 0x22222222,
                0x33333333,
            ])
        );
    }

    #[test]
    fn format_upper_hex() {
        let expected_hexstr = "3333333322222222111111110000000076543210FEDCBA9889ABCDEF01234567";
        let val = U256::from_be_hex(expected_hexstr).unwrap();
        assert_eq!(format!("{:X}", val), expected_hexstr,);
    }

    #[test]
    fn uint_overflowing_add() {
        // A non-overflowing addition
        let (sum, c) = U256::ONE.overflowing_add(&U256::ONE);
        let mut two = U256::ZERO;
        two.0[0] = 2;
        assert_eq!(sum, two);
        assert!(!c);

        // An overflowing addition
        let (sum, c) = U256::MAX.overflowing_add(&U256::ONE);
        assert_eq!(sum, U256::ZERO);
        assert!(c);
    }

    #[test]
    fn u256_overflowing_sub() {
        let (diff, b) = U256::MAX.overflowing_sub(&U256::ONE);
        let mut minus_one = U256::MAX;
        minus_one.0[0] = Word::MAX - 1;
        assert_eq!(diff, minus_one);
        assert!(!b);

        let (diff, b) = U256::ZERO.overflowing_sub(&U256::ONE);
        assert_eq!(diff, U256::MAX);
        assert!(b);
    }

    #[test]
    fn word_widening_mul() {
        let (high, low) = widening_mul(Word::MAX, Word::MAX);
        assert_eq!(high, Word::MAX - 1);
        assert_eq!(low, 1);
    }

    #[test]
    fn u256_widening_mul() {
        let (high, low) = U256::ZERO.widening_mul(&U256::MAX);
        assert_eq!(high, U256::ZERO);
        assert_eq!(low, U256::ZERO);

        let (high, low) = U256::ONE.widening_mul(&U256::ONE);
        assert_eq!(high, U256::ZERO);
        assert_eq!(low, U256::ONE);

        // U256::MAX * U256::MAX = (2 ** 256 - 1)(2 ** 256 - 1)
        //   = 2 ** 512 - 2 ** 257 + 1
        //   = (2 ** 256 - 2) * (2 ** 256) + 1
        //   = (U256::MAX - 1) * (2 ** 256) + 1
        // so high should be (U256::MAX - 1), and low should be 1
        let (high, low) = U256::MAX.widening_mul(&U256::MAX);
        assert_eq!(high, U256::MAX.overflowing_sub(&U256::ONE).0);
        assert_eq!(low, U256::ONE);
    }

    #[test]
    fn u256_add_word_inplace() {
        let mut val = U256::MAX;
        let carry = val.add_word_inplace(1, 0);
        assert!(carry);
        assert_eq!(val, U256::ZERO);
    }

    /// Test bit shifting using the following cases:
    /// shift by zero
    /// shift by one
    /// shift by word size
    /// shift by word size + some overflow
    /// shift by more than Uint::BITS
    #[test]
    fn bitshifting() {
        let max = U256::MAX;

        assert_eq!(max.overflowing_shl(0), (max, false));
        assert_eq!(max.overflowing_shr(0), (max, false));

        let (minus_one, _) = max.overflowing_sub(&U256::ONE);
        assert_eq!(max.overflowing_shl(1), (minus_one, false));
        let mut expected = U256::MAX;
        expected.0[7] = 0b01111111_11111111_11111111_11111111;
        assert_eq!(max.overflowing_shr(1), (expected, false));

        let mut expected = U256::MAX;
        expected.0[0] = 0;
        assert_eq!(max.overflowing_shl(u32::BITS as usize), (expected, false));
        let mut expected = U256::MAX;
        expected.0[7] = 0;
        assert_eq!(max.overflowing_shr(u32::BITS as usize), (expected, false));

        let mut expected = U256::MAX;
        expected.0[0] = 0;
        expected.0[1] = 0b11111111_11111111_11111111_11111110;
        assert_eq!(max.overflowing_shl(33), (expected, false));
        let mut expected = U256::MAX;
        expected.0[7] = 0;
        expected.0[6] = 0b01111111_11111111_11111111_11111111;
        assert_eq!(max.overflowing_shr(33), (expected, false));

        assert_eq!(max.overflowing_shl(U256::BITS as usize), (U256::ZERO, true));
        assert_eq!(max.overflowing_shr(U256::BITS as usize), (U256::ZERO, true));
    }

    #[test]
    fn ordering_and_equality() {
        assert!(U256::ONE >= U256::ZERO);
        assert!(U256::ZERO <= U256::ONE);
        assert!(U256::ONE == U256::ONE);
        assert!(U256::ONE != U256::ZERO);

        assert_eq!(U256::ONE.cmp(&U256::ZERO), Ordering::Greater);
        assert!(U256::ZERO < U256::ONE);
    }

    #[test]
    fn word_leading_zeros() {
        for cnt in 0..32 {
            let (shifted, _) = Word::MAX.overflowing_shr(cnt);
            assert_eq!(leading_zeros(shifted), cnt);
        }
    }

    #[test]
    fn uint_leading_zeros() {
        assert_eq!(U256::MAX.leading_zeros(), 0);
        assert_eq!(U256::ONE.leading_zeros(), 255);
        assert_eq!(U256::ZERO.leading_zeros(), 256);
    }

    #[test]
    fn uint_div_rem() {
        let (quo, rem) = U256::MAX.div_rem_vartime(&U256::ONE);
        assert_eq!(quo, U256::MAX);
        assert_eq!(rem, U256::ZERO);

        let (two, _) = U256::ONE.overflowing_shl(1);
        let (quo, rem) = U256::MAX.div_rem_vartime(&two);
        let (half, _) = U256::MAX.overflowing_shr(1);
        assert_eq!(quo, half);
        assert_eq!(rem, U256::ONE);
    }
}
