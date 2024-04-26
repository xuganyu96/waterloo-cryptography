use primitives::Poly;

fn main() {
    let poly = Poly::from_coeffs([3328; 256]);
    println!("{:x}", poly);
}
