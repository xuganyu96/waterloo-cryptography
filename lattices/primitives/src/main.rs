use primitives::Poly;
use sha3::{
    digest::{ExtendableOutput, Update},
    Shake256,
};

fn main() {
    let mut xof = Shake256::default();
    xof.update(b"!!!Hello, world");
    let mut xof = xof.finalize_xof();
    let poly = Poly::sample_cbd_eta3(&mut xof);
    println!("{}", poly.polymul(&poly));
}
