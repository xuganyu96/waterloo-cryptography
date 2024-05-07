use primitives::algebra::PolyNTT;
use sha3::{
    digest::{ExtendableOutput, Update},
    Shake256,
};

fn main() {
    let mut xof = Shake256::default();
    xof.update(b"test");
    let mut xof = xof.finalize_xof();
    let poly = PolyNTT::sample_uniform(&mut xof).invert_ntt();
    println!("{:b}", poly);

    let mut buffer = [0u8; 32 * 12];
    primitives::byte_encode(poly.as_coeffs(), 12, &mut buffer);

    for byte in buffer {
        print!("{:08b}, ", byte);
    }
}
