//! A naive implementation of a cipher based on the LWE problem:
//! https://cims.nyu.edu/~regev/papers/lwesurvey.pdf
//!
//! No big integer type is used; instead, u128 is used to reduce 
//! implementation complexity at the cost of security

/// The security parameter
pub struct NaiveLweSecParams {
    /// The number of bits 
    keysize: u128,
}

pub struct NaiveLweParams {
    modulo: u128,
    nsamples: u128,
    noise_factor: f64,
}

impl NaiveLweParams {
    /// Generate a prime number between n^2 and 2 * n^2
    fn primegen(n: u128) -> u128 {
        todo!();
    }

    fn nsamplegen(n: u128, modulo: u128) -> u128 {
        todo!();
    }

    fn alphagen(n: u128) -> f64 {
        todo!();
    }

    /// Use the security parameter "n" to generate the parameters: q, m, and
    /// alpha. Based on the survey, 
    pub fn paramsgen(sec_params: NaiveLweSecParams) -> NaiveLweParams {
        let n = sec_params.keysize;
        let modulo = Self::primegen(n);
        let nsamples = Self::nsamplegen(n, modulo);
        let noise_factor = Self::alphagen(n);

        return NaiveLweParams { modulo, nsamples, noise_factor }
    }
}

pub struct NaiveLwePubKey {
    // TODO: samples should be a matrix with shape (nsamples, keysize)
    // TODO: observations (obs) should be a matrix with shape (nsamples, 1)
}

pub struct NaiveLweSecKey {
    /// TODO: the secret is a matrix with shape (keysize, 1)
    secret: Vec<u64>,
}

pub struct NaiveLweCipher {
    params: NaiveLweParams,

    /// The cipher always has access to a public key
    pubkey: NaiveLwePubKey,

    /// The cipher does not always have access to a secret key
    seckey: Option<NaiveLweSecKey>,
}


impl NaiveLweCipher {

    pub fn keygen(params: NaiveLweParams) -> (NaiveLwePubKey, NaiveLweSecKey) {
        todo!();
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        todo!();
    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        todo!();
    }
}
