//! Functions used for breaking Vigenere cipher
//! Assume that the plaintext is English
use crate::arithmetics;
use std::collections::HashMap;

/// The homework version of index of coincidence is missing a constant c = 26;
/// after multiplying the constant, the value matches what's shown on Wikipedia:
/// https://en.wikipedia.org/wiki/Index_of_coincidence
pub fn index_of_coincidence(text: &str) -> f64 {
    let textlen = text.chars().filter(|c| c.is_alphabetic()).count();

    let mut charcounts: HashMap<char, usize> = HashMap::new();
    text.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
        let count = charcounts.get(&c).unwrap_or(&0);
        charcounts.insert(c, count + 1);
    });

    let numerator = charcounts
        .iter()
        .map(|(_c, count)| arithmetics::combination(*count, 2))
        .sum::<usize>();
    let denominator = arithmetics::combination(textlen, 2);

    return (numerator as f64) / (denominator as f64);
}

/// Return a copy of the input plaintext but sliced acccording to block size
/// and offset
fn copy_alignment(bytes: &[u8], blocksize: usize, offset: usize) -> Vec<u8> {
    let mut output = vec![];

    let mut i = offset;
    while i < bytes.len() {
        output.push(*bytes.get(i).unwrap());
        i += blocksize;
    }

    return output;
}

/// For each of the sub-ciphertext, compute the IoC, the score is the MSE
/// between subciphertext's IoC's against the reference IoC
fn score_blocksize(
    ciphertext: &[u8],
    reference_ioc: f64,
    blocksize: usize,
) -> f64 {
    let mut mse = 0.;
    for offset in 0..blocksize {
        let subciphertext = copy_alignment(ciphertext, blocksize, offset);
        let subciphertext_str = String::from_utf8(subciphertext).unwrap();
        let sub_ioc = index_of_coincidence(&subciphertext_str);
        mse += (reference_ioc - sub_ioc) * (reference_ioc - sub_ioc);
    }

    return mse / (blocksize as f64);
}

/// Search all possible block sizes. For each block size, compute the index of
/// coincidence and and average distance to the reference index of coincidence.
/// Rank the block sizes based on the distance to the refernece IoC
pub fn search_blocksize(
    ciphertext: &[u8],
    reference_ioc: f64,
    max_blocksize: usize,
) -> Vec<(usize, f64)> {
    let mut blocksizes = (1..=max_blocksize)
        .map(|blocksize| {
            let dist = score_blocksize(ciphertext, reference_ioc, blocksize);
            return (blocksize, dist);
        })
        .collect::<Vec<(usize, f64)>>();

    blocksizes.sort_by(|elem1, elem2| {
        let (_, mse1) = elem1;
        let (_, mse2) = elem2;
        return mse1.partial_cmp(mse2).unwrap();
    });

    return blocksizes;
}
