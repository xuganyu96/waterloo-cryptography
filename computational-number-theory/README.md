# Computational number theory
Rust implementation of algorithms described in [A Course in Computational Number Theory](https://link.springer.com/book/10.1007/978-3-662-02945-9) by Henri Cohen.

A first batch of APIs:
- [x] Overflowing addition
- [x] Overflowing subtraction
- [x] Widening multiplication using schoolbook multiplication
- [ ] Bit shifting
- [ ] Euclidean division


**Table of Content**

## Data structure
```rust
pub type Word = u32;

pub struct Uint<L: usize>([Word; L]);
```

We use `u32` for a Word, this is so that `widening_mul` can be implemented by casting `u32` into `u64`. If `u64` is used for a Word, then `widening_mul` will need `u128`, which is only supported on a limited number of platforms.

## Bit shifting
This Rust code implements the `overflowing_shl` method for a big integer type represented by the `Uint` struct. Let's break down what each part of the code does:

1. **Function Signature:**
   ```rust
   pub fn overflowing_shl(&self, shift: usize) -> (Self, bool)
   ```
   - `&self`: Indicates that this method takes a reference to the `Uint` struct as input.
   - `shift: usize`: Specifies the amount by which the big integer should be left-shifted.
   - `-> (Self, bool)`: Indicates that the method returns a tuple containing the shifted big integer (`Self`) and a boolean value indicating whether an overflow occurred.

2. **Initialization:**
   ```rust
   let mut shifted = Self::ZERO;
   ```
   - Initializes a new `Uint` variable called `shifted` with all elements set to zero. `Self::ZERO` is likely a constant or static method that creates a zero-initialized `Uint`.

3. **Shift Amount Calculation:**
   ```rust
   let word_shift = shift / (Word::BITS as usize);
   let overflow = shift % (Word::BITS as usize);
   ```
   - Calculates the number of whole words (`word_shift`) to shift by dividing the input shift amount by the number of bits in a word (`Word::BITS`).
   - Calculates the remaining bit count to shift within a word (`overflow`) by taking the modulo of the shift amount with the number of bits in a word.

4. **Shifting Logic:**
   ```rust
   for i in 0..L {
       if i + word_shift < L {
           shifted.0[i + word_shift] |= self.0[i] << overflow;
       }
       if i + word_shift + 1 < L && (overflow != 0) {
           shifted.0[i + word_shift + 1] |= self.0[i] >> (Word::BITS as usize - overflow);
       }
   }
   ```
   - Iterates through each element (`i`) of the input `Uint` (`self`) from index `0` to `L-1`.
   - Shifts each element left by `overflow` bits and bitwise ORs it with the corresponding element in the `shifted` big integer (`shifted.0[i + word_shift] |= self.0[i] << overflow`).
   - Handles potential overflow into the next element by shifting right and ORing with the next element (`shifted.0[i + word_shift + 1] |= self.0[i] >> (Word::BITS as usize - overflow)`).

5. **Overflow Check:**
   ```rust
   return (shifted, shift >= (Self::BITS as usize));
   ```
   - Checks if the input shift amount is greater than or equal to the total number of bits in the big integer (`Self::BITS`) and returns `true` if an overflow occurred, `false` otherwise.

Overall, this code performs a left shift operation on a big integer represented by the `Uint` struct, handling both intra-word shifts and inter-word shifts while also checking for overflow conditions.