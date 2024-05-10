//! Hash, XOF, and PRF
use crate::SEEDSIZE;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Digest, Sha3_256, Sha3_512, Shake128, Shake256,
};

/// Hash function H is Sha3-256(s)
pub fn hash_h(stream: &[u8]) -> [u8; SEEDSIZE] {
    let mut output = [0u8; SEEDSIZE];
    let mut hasher = Sha3_256::new();
    Digest::update(&mut hasher, stream);
    let result = hasher.finalize();
    output.clone_from_slice(&result);

    return output;
}

/// Hash function J is Shake256(s)
pub fn hash_j(stream: &[u8]) -> [u8; SEEDSIZE] {
    let mut output = [0u8; SEEDSIZE];

    let mut hasher = Shake256::default();
    hasher.update(stream);
    let mut xof = hasher.finalize_xof();
    xof.read(&mut output);

    return output;
}

/// Hash function G uses SHA3-512 but output 2 32-bytes
pub fn hash_g(stream: &[u8]) -> ([u8; SEEDSIZE], [u8; SEEDSIZE]) {
    let mut hasher = Sha3_512::new();
    Digest::update(&mut hasher, stream);
    let hash = hasher.finalize();

    let mut first = [0u8; SEEDSIZE];
    let mut second = [0u8; SEEDSIZE];

    first.clone_from_slice(hash.get(0..SEEDSIZE).expect("Unexpected out-of-bound"));
    second.clone_from_slice(
        hash.get(SEEDSIZE..(2 * SEEDSIZE))
            .expect("Unexpected out-of-bound"),
    );

    return (first, second);
}

pub fn shake256_prf<const ETA: usize>(seed: [u8; SEEDSIZE], ctr: u8) -> [u8; ETA] {
    let mut hasher = Shake256::default();
    hasher.update(&seed);
    hasher.update(&[ctr]);
    let mut xof = hasher.finalize_xof();

    let mut output = [0u8; ETA];
    xof.read(&mut output);
    return output;
}

pub fn shake128_xof(seed: [u8; SEEDSIZE], i: u8, j: u8) -> impl XofReader {
    let mut hasher = Shake128::default();
    hasher.update(&seed);
    hasher.update(&[i, j]);
    let xof = hasher.finalize_xof();

    return xof;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple sanity check
    #[test]
    fn hash_sanity() {
        let seed = b"Hello, world";
        assert_eq!(
            hash_h(seed),
            [
                0x35, 0x50, 0xab, 0xa9, 0x74, 0x92, 0xde, 0x38, 0xaf, 0x30, 0x66, 0xf0, 0x15, 0x7f,
                0xc5, 0x32, 0xdb, 0x67, 0x91, 0xb3, 0x7d, 0x53, 0x26, 0x2c, 0xe7, 0x68, 0x8d, 0xcc,
                0x5d, 0x46, 0x18, 0x56
            ],
        );

        assert_eq!(
            hash_j(seed),
            [
                0x87, 0x01, 0xdf, 0x7c, 0x04, 0xe1, 0xed, 0xfd, 0x40, 0xbb, 0xc1, 0x25, 0x37, 0x89,
                0xc9, 0x93, 0x85, 0xa4, 0xc8, 0xa7, 0x0b, 0x11, 0x6e, 0xab, 0x78, 0x51, 0xe7, 0x63,
                0xeb, 0x63, 0x69, 0xbc
            ],
        );
        assert_eq!(
            hash_g(seed),
            (
                [
                    0x96, 0x32, 0x03, 0x43, 0xe6, 0x22, 0x84, 0x77, 0xae, 0xae, 0xdf, 0x63, 0x1d,
                    0xbf, 0xb8, 0x29, 0xd9, 0x43, 0xcf, 0x3e, 0xbd, 0xae, 0xfa, 0x1a, 0x5e, 0xbd,
                    0x76, 0x86, 0x02, 0xa7, 0x15, 0x61
                ],
                [
                    0x71, 0x07, 0xc1, 0x89, 0x59, 0xa5, 0xd7, 0x82, 0x66, 0xab, 0x5d, 0x57, 0x76,
                    0xdf, 0x62, 0xd3, 0xdc, 0x92, 0x9d, 0x9a, 0x6e, 0x1f, 0x40, 0x40, 0xaa, 0xbf,
                    0x68, 0xf7, 0x56, 0xeb, 0x7c, 0x56
                ]
            )
        );
    }
}
