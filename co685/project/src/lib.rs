//! A Rust implementation of the Benaloh cryptosystem
//!
//! I will refrain from implementing arithmetic primitives like big prime
//! generation, modexp, and other things by myself. Instead a mature
//! cryptographic library such as "ring" or "rustcrypto" will be used. At this
//! moment I am inclined toward "ring", although time-permitting I would like
//! to learn rustcrypto crates, as well.
use ring::rand::{SecureRandom, SystemRandom};

// So how does briansmith/ring work with:
// 1. Generating random bits
// 2. Primality test
// 3. Modulo multiplication (n = pq)
// 4. Modulo exponentiation (m^e mod p)


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_bytes() {
        let mut randbits = [0u8; 32];  // that's 256 bits
        let sysrand = SystemRandom::new();
        let _ = sysrand.fill(&mut randbits).unwrap();
    }
}
