use crate::{field::Field, goldilocks::GoldilocksField, poseidon::calculate_arbitrary_poseidon};

mod goldilocks;
mod utils;
mod field;
mod poseidon;
mod constants;

fn main() {
    let inputs = [
        GoldilocksField::from_canonical_u64(104),
        GoldilocksField::from_canonical_u64(101),
        GoldilocksField::from_canonical_u64(108),
        GoldilocksField::from_canonical_u64(108),
        GoldilocksField::from_canonical_u64(111),
        GoldilocksField::from_canonical_u64(119),
        GoldilocksField::from_canonical_u64(111),
        GoldilocksField::from_canonical_u64(114),
        GoldilocksField::from_canonical_u64(108),
        GoldilocksField::from_canonical_u64(100),
    ];
    let res = calculate_arbitrary_poseidon(&inputs);
    println!("{:?}", res);
}
