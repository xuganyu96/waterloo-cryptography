//! My implementation of computational number theory
//! Follows Henri Cohen's [A Course in Computational Algebraic Number Theory]

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

/// A big integer is an array of words
/// We will tentatively use little-endian: lower index encodes less significant digits
/// num = sum_{i=0}^{L-1}(Uint[i] * (Word ** i))
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Uint<const L: usize>([Word; L]);

impl<const L: usize> Uint<L> {
    pub const ZERO: Self = Self([0; L]);
    pub const MAX: Self = Self([Word::MAX; L]);

    /// There is no easy way to specify a "1" using just the array notation, so we use a const fn
    pub const fn one() -> Self {
        let mut arr = [0; L];
        arr[0] = 1;
        return Self(arr);
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
        let (mut high, mut low) = ([0; L], [0; L]);
        let mut self_loc = 0;

        while self_loc < L {
            let mut other_loc = 0;
            while other_loc < L {
                todo!("need Word::widening_mul");

                other_loc += 1;
            }
            self_loc += 1;
        }

        return (Self(high), Self(low));
    }
}

pub type U256 = Uint<8>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uint_overflowing_add() {
        // A non-overflowing addition
        let (sum, c) = U256::one().overflowing_add(&U256::one());
        let mut two = U256::ZERO;
        two.0[0] = 2;
        assert_eq!(sum, two);
        assert!(!c);

        // An overflowing addition
        let (sum, c) = U256::MAX.overflowing_add(&U256::one());
        assert_eq!(sum, U256::ZERO);
        assert!(c);
    }

    #[test]
    fn u256_overflowing_sub() {
        let (diff, b) = U256::MAX.overflowing_sub(&U256::one());
        let mut minus_one = U256::MAX;
        minus_one.0[0] = Word::MAX - 1;
        assert_eq!(diff, minus_one);
        assert!(!b);

        let (diff, b) = U256::ZERO.overflowing_sub(&U256::one());
        assert_eq!(diff, U256::MAX);
        assert!(b);
    }

    #[test]
    fn word_widening_mul() {
        let (high, low) = widening_mul(Word::MAX, Word::MAX);
        let expected_prod = (Word::MAX as LongWord) * (Word::MAX as LongWord);
        assert_eq!(expected_prod as Word, low);
        assert_eq!((expected_prod >> 32) as Word, high);
    }
}
