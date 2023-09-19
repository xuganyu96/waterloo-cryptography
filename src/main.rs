use std::fs;
use waterloo_crypto::vigenere;

fn main() {
    let text = fs::read_to_string("inputs/q1-mobydick.txt").unwrap();
    let reference_ioc = vigenere::index_of_coincidence(&text);
    println!("{}", reference_ioc * 26.);

    let ciphertext = fs::read_to_string("inputs/q1-ciphertext.txt").unwrap();
    let blocksizes =
        vigenere::search_blocksize(ciphertext.as_bytes(), reference_ioc, 100);
    blocksizes.iter().for_each(|(blocksize, score)| {
        println!("{blocksize}: {score}");
    })
}
