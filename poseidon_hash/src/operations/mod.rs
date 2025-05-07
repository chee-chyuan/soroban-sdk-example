use elem::Elem;
use poseidon_const::{M_INT_DIAG_HZN, ROUNDS_HALF_FULL, ROUNDS_PARTIAL, ROUND_CONSTANTS};

pub const CELLS_OUT: usize = 8;
pub const DIGEST_WORDS: usize = 8;
pub const CELLS: usize = 24;

/// The 'rate' of the sponge, i.e. how much we can safely add/remove per mixing.
pub const CELLS_RATE: usize = 16;

pub mod elem;
pub mod ext_elem;
pub mod poseidon_const;
pub mod digest;

pub type BabyBearElem = Elem;

pub const BABY_BEAR_ELEM_ZERO: BabyBearElem = Elem::new(0);
/// The raw sponge mixing function
pub fn poseidon2_mix(cells: &mut [BabyBearElem; CELLS]) {
    let mut round = 0;

    // First linear layer.
    multiply_by_m_ext(cells);
    // tracing::trace!("After initial mExt: {cells:?}");

    // Do initial full rounds
    for _i in 0..ROUNDS_HALF_FULL {
        full_round(cells, round);
        round += 1;
    }
    // Do partial rounds
    for _i in 0..ROUNDS_PARTIAL {
        partial_round(cells, round);
        round += 1;
    }
    // tracing::trace!("After partial rounds: {cells:?}");
    // Do remaining full rounds
    for _i in 0..ROUNDS_HALF_FULL {
        full_round(cells, round);
        round += 1;
    }
}

fn multiply_by_4x4_circulant(x: &[BabyBearElem; 4]) -> [BabyBearElem; 4] {
    // See appendix B of Poseidon2 paper.
    let t0 = x[0] + x[1];
    let t1 = x[2] + x[3];
    let t2 = BabyBearElem::new(2) * x[1] + t1;
    let t3 = BabyBearElem::new(2) * x[3] + t0;
    let t4 = BabyBearElem::new(4) * t1 + t3;
    let t5 = BabyBearElem::new(4) * t0 + t2;
    let t6 = t3 + t5;
    let t7 = t2 + t4;
    [t6, t5, t7, t4]
}

fn multiply_by_m_ext(cells: &mut [BabyBearElem; CELLS]) {
    // Optimized method for multiplication by M_EXT.
    // See appendix B of Poseidon2 paper for additional details.
    let old_cells = *cells;
    cells.fill(BABY_BEAR_ELEM_ZERO);
    let mut tmp_sums = [BABY_BEAR_ELEM_ZERO; 4];

    for i in 0..CELLS / 4 {
        let chunk_array: [BabyBearElem; 4] = [
            old_cells[i * 4],
            old_cells[i * 4 + 1],
            old_cells[i * 4 + 2],
            old_cells[i * 4 + 3],
        ];
        let out = multiply_by_4x4_circulant(&chunk_array);
        for j in 0..4 {
            tmp_sums[j] += out[j];
            cells[i * 4 + j] += out[j];
        }
    }
    for i in 0..CELLS {
        cells[i] += tmp_sums[i % 4];
    }
}

fn full_round(cells: &mut [BabyBearElem; CELLS], round: usize) {
    add_round_constants_full(cells, round);
    // if round == 0 {
    //     tracing::trace!("After constants in full round 0: {cells:?}");
    // }

    do_full_sboxes(cells);
    multiply_by_m_ext(cells);
    // tracing::trace!("After mExt in full round {round}: {cells:?}");
}

fn partial_round(cells: &mut [BabyBearElem; CELLS], round: usize) {
    add_round_constants_partial(cells, round);
    do_partial_sboxes(cells);
    multiply_by_m_int(cells);
}

fn add_round_constants_full(cells: &mut [BabyBearElem; CELLS], round: usize) {
    for i in 0..CELLS {
        cells[i] += ROUND_CONSTANTS[round * CELLS + i];
    }
}

fn add_round_constants_partial(cells: &mut [BabyBearElem; CELLS], round: usize) {
    cells[0] += ROUND_CONSTANTS[round * CELLS];
}

fn do_full_sboxes(cells: &mut [BabyBearElem; CELLS]) {
    for cell in cells.iter_mut() {
        *cell = sbox(*cell);
    }
}

fn do_partial_sboxes(cells: &mut [BabyBearElem; CELLS]) {
    cells[0] = sbox(cells[0]);
}

fn multiply_by_m_int(cells: &mut [BabyBearElem; CELLS]) {
    // Exploit the fact that off-diagonal entries of M_INT are all 1.
    let sum: BabyBearElem = cells.iter().fold(BABY_BEAR_ELEM_ZERO, |acc, x| acc + *x);
    for i in 0..CELLS {
        cells[i] = sum + M_INT_DIAG_HZN[i] * cells[i];
    }
}

fn sbox(x: BabyBearElem) -> BabyBearElem {
    let x2 = x * x;
    let x4 = x2 * x2;
    let x6 = x4 * x2;
    x6 * x
}
