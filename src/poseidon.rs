use std::cmp::min;

use crate::{
    constants::*,
    field::{Field, Square},
    goldilocks::GoldilocksField,
};

pub const POSEIDON_STATE_WIDTH: usize = 12;
pub const POSEIDON_INPUT_NUM: usize = 12;
pub const POSEIDON_OUTPUT_NUM: usize = 12;

fn constant_layer_field(state: &mut [GoldilocksField; 12], round_ctr: usize) {
    for i in 0..12 {
        state[i] += GoldilocksField::from_canonical_u64(ALL_ROUND_CONSTANTS[i + 12 * round_ctr]);
    }
}

fn sbox_monomial(x: GoldilocksField) -> GoldilocksField {
    let x2 = x.square();
    let x4 = x2.square();
    let x3 = x * x2;
    x3 * x4
}

fn sbox_layer_field(state: &mut [GoldilocksField; POSEIDON_STATE_WIDTH]) {
    for i in 0..POSEIDON_STATE_WIDTH {
        state[i] = sbox_monomial(state[i]);
    }
}

fn mds_row_shf_field(r: usize, v: &[GoldilocksField; POSEIDON_STATE_WIDTH]) -> GoldilocksField {
    let mut res = GoldilocksField::ZERO;
    for i in 0..POSEIDON_STATE_WIDTH {
        res += v[(i + r) % POSEIDON_STATE_WIDTH]
            * GoldilocksField::from_canonical_u64(MDS_MATRIX_CIRC[i]);
    }
    res += v[r] * GoldilocksField::from_canonical_u64(MDS_MATRIX_DIAG[r]);
    res
}

fn mds_layer_field(
    state: &[GoldilocksField; POSEIDON_STATE_WIDTH],
) -> [GoldilocksField; POSEIDON_STATE_WIDTH] {
    let mut res = [GoldilocksField::ZERO; POSEIDON_STATE_WIDTH];
    for i in 0..POSEIDON_STATE_WIDTH {
        res[i] = mds_row_shf_field(i, &state);
    }
    res
}

fn partial_first_constant_layer(state: &mut [GoldilocksField; POSEIDON_STATE_WIDTH]) {
    for i in 0..12 {
        if i < POSEIDON_STATE_WIDTH {
            state[i] += GoldilocksField::from_canonical_u64(FAST_PARTIAL_FIRST_ROUND_CONSTANT[i]);
        }
    }
}

fn mds_partial_layer_init(
    state: &[GoldilocksField; POSEIDON_STATE_WIDTH],
) -> [GoldilocksField; POSEIDON_STATE_WIDTH] {
    let mut result = [GoldilocksField::ZERO; POSEIDON_STATE_WIDTH];
    result[0] = state[0];
    for r in 1..12 {
        if r < POSEIDON_STATE_WIDTH {
            for c in 1..12 {
                if c < POSEIDON_STATE_WIDTH {
                    let t = GoldilocksField::from_canonical_u64(
                        FAST_PARTIAL_ROUND_INITIAL_MATRIX[r - 1][c - 1],
                    );
                    result[c] += state[r] * t;
                }
            }
        }
    }
    result
}

fn mds_partial_layer_fast_field(
    state: &[GoldilocksField; POSEIDON_STATE_WIDTH],
    r: usize,
) -> [GoldilocksField; POSEIDON_STATE_WIDTH] {
    let s0 = state[0];
    let mds0to0 = MDS_MATRIX_CIRC[0] + MDS_MATRIX_DIAG[0];
    let mut d = s0 * GoldilocksField::from_canonical_u64(mds0to0);
    for i in 1..POSEIDON_STATE_WIDTH {
        let t = GoldilocksField::from_canonical_u64(FAST_PARTIAL_ROUND_W_HATS[r][i - 1]);
        d += state[i] * t;
    }
    let mut result = [GoldilocksField::ZERO; POSEIDON_STATE_WIDTH];
    result[0] = d;
    for i in 1..POSEIDON_STATE_WIDTH {
        let t = GoldilocksField::from_canonical_u64(FAST_PARTIAL_ROUND_VS[r][i - 1]);
        result[i] = state[0] * t + state[i];
    }
    result
}

fn calculate_poseidon(
    full_input: [GoldilocksField; POSEIDON_INPUT_NUM],
) -> [GoldilocksField; POSEIDON_OUTPUT_NUM] {
    let mut state = full_input;
    let mut round_ctr = 0;

    // First set of full rounds.
    (0..HALF_N_FULL_ROUNDS).for_each(|_| {
        constant_layer_field(&mut state, round_ctr);
        sbox_layer_field(&mut state);
        state = mds_layer_field(&state);
        round_ctr += 1;
    });

    // Partial rounds.
    partial_first_constant_layer(&mut state);
    state = mds_partial_layer_init(&state);
    for r in 0..(N_PARTIAL_ROUNDS - 1) {
        let sbox_in = state[0];
        state[0] = sbox_monomial(sbox_in);
        state[0] += GoldilocksField::from_canonical_u64(FAST_PARTIAL_ROUND_CONSTANTS[r]);
        state = mds_partial_layer_fast_field(&state, r);
    }
    let sbox_in = state[0];
    state[0] = sbox_monomial(sbox_in);
    state = mds_partial_layer_fast_field(&state, N_PARTIAL_ROUNDS - 1);
    round_ctr += N_PARTIAL_ROUNDS;

    // Second set of full rounds.
    for _ in 0..HALF_N_FULL_ROUNDS {
        constant_layer_field(&mut state, round_ctr);
        sbox_layer_field(&mut state);
        state = mds_layer_field(&state);
        round_ctr += 1;
    }

    state
}

pub fn calculate_arbitrary_poseidon(inputs: &[GoldilocksField]) -> [GoldilocksField; 4] {
    let mut state: [GoldilocksField; POSEIDON_STATE_WIDTH] =
        [GoldilocksField::ZERO; POSEIDON_STATE_WIDTH];

    for input_chunk in inputs.chunks(8) {
        let end = min(input_chunk.len(), 8);
        state[0..end].copy_from_slice(&input_chunk[0..end]);
        state = calculate_poseidon(state);
    }
    state[0..4].try_into().expect("slice with incorrect length")
}

pub fn poseidon_u64(inputs: &[u64]) -> [u64; 4] {
    let fields: Vec<GoldilocksField> = inputs
        .iter()
        .map(|x| GoldilocksField::from_canonical_u64(*x))
        .collect();
    let res = calculate_arbitrary_poseidon(&fields);
    res.map(|f| f.0)
}

pub fn poseidon_u64_bytes(bytes: &[u8]) -> [u64; 4] {
    let inputs = bytes_to_u64s(bytes.to_vec());
    poseidon_u64(&inputs)
}

pub fn poseidon_u64_for_bytes(inputs: &[u64]) -> [u8; 32] {
    let res = poseidon_u64(inputs);
    u64s_to_bytes(&res)
        .try_into()
        .expect("slice with incorrect length")
}

pub fn poseidon_u64_bytes_for_bytes(bytes: &[u8]) -> [u8; 32] {
    let res = poseidon_u64_bytes(bytes);
    //println!("{:?}", res);
    u64s_to_bytes(&res)
        .try_into()
        .expect("slice with incorrect length")
}

fn u64s_to_bytes(arr: &[u64]) -> Vec<u8> {
    arr.iter().flat_map(|w| w.to_be_bytes()).collect()
}

fn bytes_to_u64s(bytes: Vec<u8>) -> Vec<u64> {
    assert!(bytes.len() % 8 == 0, "Bytes must be divisible by 8");
    bytes
        .chunks(8)
        .map(|chunk| {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(chunk);
            u64::from_be_bytes(bytes)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{poseidon_u64, poseidon_u64_bytes_for_bytes};

    #[test]
    fn test_poseidon_u64() {
        let inputs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let hash = poseidon_u64(&inputs);
        assert_eq!(
            hash,
            [
                3619294218193778203,
                11297464610014100521,
                916593713866203210,
                11797937562375563491
            ]
        )
    }

    #[test]
    fn test_poseidon_bytes() {
        let bytes = [
            0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let hash = poseidon_u64_bytes_for_bytes(&bytes);
        assert_eq!(
            hash,
            [
                85, 160, 96, 159, 172, 88, 72, 90, 174, 246, 73, 172, 217, 75, 152, 86, 110, 234,
                34, 143, 235, 106, 165, 129, 158, 75, 134, 226, 30, 7, 236, 120
            ]
        )
    }
}
