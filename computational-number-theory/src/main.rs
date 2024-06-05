use compnt::U256;

fn main() {
    let mut val = U256::ONE;
    for _ in 0..40 {
        println!("{:?}", val);
        let (next_val, overflowing) = val.overflowing_add(&val);
        val = next_val;
    }
}
