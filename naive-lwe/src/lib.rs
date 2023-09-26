/// The security parameter
pub struct NaiveLweSecParams {
    /// The number of bits 
    keysize: usize,
}

pub struct NaiveLweParams {
    modulo: usize,
    nsamples: usize,
    noise_factor: f64,
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
    pub fn paramsgen(sec_params: NaiveLweSecParams) -> NaiveLweParams {
        todo!();
    }

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
