use compnt::U256;

fn main() {
    let (quo, rem) = U256::MAX.div_rem_vartime(&U256::ONE);
    println!("{:X}, {:X}", quo, rem);
}
