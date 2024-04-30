use primitives::Poly;

fn main() {
    let poly = Poly::from_words([3328; 256]);
    println!("{:x}", poly);
}
