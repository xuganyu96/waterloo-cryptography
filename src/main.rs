use std::collections::HashMap;
use std::fs;

fn rank_frequencies(ciphertext_str: &str) -> Vec<f64> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut frequencies: Vec<f64> = vec![];

    ciphertext_str.chars().for_each(|char_| {
        let count = counts.get(&char_).unwrap_or(&0);
        counts.insert(char_, *count + 1);
    });

    counts.iter().for_each(|(_, count)| {
        let frequency = (*count as f64) / (ciphertext_str.len() as f64);
        frequencies.push(frequency);
    });
    frequencies.sort_by(|f1, f2| f2.partial_cmp(f1).unwrap());
    while frequencies.len() < 26 {
        frequencies.push(0.0);
    }

    return frequencies;
}

fn main() {
    let ciphertexts_frequencies = (0..4)
        .map(|i| {
            let path = format!("inputs/a1q2ciphertexts/ctxt{}.txt", i);
            let ciphertext_str = fs::read_to_string(path).unwrap();
            let frequencies = rank_frequencies(&ciphertext_str);
            return frequencies;
        })
        .collect::<Vec<Vec<f64>>>();

    for a in 0..26 {
        for i in 0..4 {
            let frequencies = ciphertexts_frequencies.get(i).unwrap();
            let frequency = frequencies.get(a).unwrap();
            print!("{frequency}|");
        }
        println!();
    }
}
