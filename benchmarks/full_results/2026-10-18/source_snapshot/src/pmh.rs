//! CNOT-circuit synthesis.
//!
//! Corollary 9 of the paper puts a circuit into canonical form (Theorem 8)
//! and then shrinks each CNOT round with the Patel–Markov–Hayes algorithm
//! (quant-ph/0302002), which needs O(n²/log n) gates. Plain Gauss–Jordan
//! elimination, which needs O(n²), is provided for comparison.
//!
//! Both algorithms return *row operations* (c, t), meaning "row t ⊕= row c",
//! that reduce an invertible matrix A to the identity.

use crate::error::{value_err, Result};
use crate::gf2::BitMatrix;
use std::collections::HashMap;

/// The section size PMH recommends: about ½ log₂ n, at least 1.
pub fn default_section_size(n: usize) -> usize {
    if n < 2 {
        return 1;
    }
    (((n as f64).log2() / 2.0).round() as usize).max(1)
}

fn check_invertible(a: &BitMatrix) -> Result<()> {
    if a.rows() != a.cols() || a.inverse().is_none() {
        return value_err("matrix must be square and invertible over GF(2)");
    }
    Ok(())
}

/// Gauss–Jordan with row additions only (no swaps).
pub fn gauss_row_ops(a: &BitMatrix) -> Result<Vec<(usize, usize)>> {
    check_invertible(a)?;
    let mut a = a.clone();
    let n = a.rows();
    let mut ops = Vec::new();
    for col in 0..n {
        if !a.get(col, col) {
            let r = (col + 1..n)
                .find(|&r| a.get(r, col))
                .expect("invertible matrix has a pivot");
            a.row_xor(col, r);
            ops.push((r, col));
        }
        for r in 0..n {
            if r != col && a.get(r, col) {
                a.row_xor(r, col);
                ops.push((col, r));
            }
        }
    }
    debug_assert!(a.is_identity());
    Ok(ops)
}

/// Patel–Markov–Hayes with section size `m` (1 ≤ m ≤ 63).
pub fn pmh_row_ops(a: &BitMatrix, m: usize) -> Result<Vec<(usize, usize)>> {
    check_invertible(a)?;
    if !(1..=63).contains(&m) {
        return value_err("section size must be in 1..=63");
    }
    let mut a = a.clone();
    let mut ops = lower_synth(&mut a, m);
    let mut at = a.transpose();
    let upper = lower_synth(&mut at, m);
    debug_assert!(at.is_identity());
    // `upper` reduces Aᵀ; transposing each op and reversing the order
    // reduces A itself.
    ops.extend(upper.iter().rev().map(|&(c, t)| (t, c)));
    Ok(ops)
}

/// Clears everything below the diagonal, section by section, first removing
/// duplicate sub-rows within each section (the source of PMH's log n saving).
fn lower_synth(a: &mut BitMatrix, m: usize) -> Vec<(usize, usize)> {
    let n = a.rows();
    let mut ops = Vec::new();
    for start in (0..n).step_by(m) {
        let end = (start + m).min(n);
        let pattern = |a: &BitMatrix, row: usize| -> u64 {
            (start..end).fold(0u64, |acc, c| acc << 1 | a.get(row, c) as u64)
        };
        let mut seen: HashMap<u64, usize> = HashMap::new();
        for row in start..n {
            let p = pattern(a, row);
            if p == 0 {
                continue;
            }
            match seen.get(&p) {
                Some(&first) => {
                    a.row_xor(row, first);
                    ops.push((first, row));
                }
                None => {
                    seen.insert(p, row);
                }
            }
        }
        for col in start..end {
            let mut diag_one = a.get(col, col);
            for row in col + 1..n {
                if a.get(row, col) {
                    if !diag_one {
                        a.row_xor(col, row);
                        ops.push((row, col));
                        diag_one = true;
                    }
                    a.row_xor(row, col);
                    ops.push((col, row));
                }
            }
        }
    }
    ops
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    pub(crate) fn random_invertible(n: usize, rng: &mut Rng) -> BitMatrix {
        loop {
            let mut a = BitMatrix::zeros(n, n);
            for i in 0..n {
                for j in 0..n {
                    a.set(i, j, rng.bit());
                }
            }
            if a.inverse().is_some() {
                return a;
            }
        }
    }

    fn apply(a: &BitMatrix, ops: &[(usize, usize)]) -> BitMatrix {
        let mut a = a.clone();
        for &(c, t) in ops {
            a.row_xor(t, c);
        }
        a
    }

    #[test]
    fn both_reduce_to_identity() {
        let mut rng = Rng::seed_from_u64(5);
        for n in [1, 2, 3, 8, 17, 40] {
            let a = random_invertible(n, &mut rng);
            assert!(apply(&a, &gauss_row_ops(&a).unwrap()).is_identity());
            for m in [1, 2, 3, default_section_size(n)] {
                assert!(
                    apply(&a, &pmh_row_ops(&a, m).unwrap()).is_identity(),
                    "n={n} m={m}"
                );
            }
        }
    }

    #[test]
    fn pmh_is_shorter_for_large_n() {
        let mut rng = Rng::seed_from_u64(9);
        let n = 200;
        let a = random_invertible(n, &mut rng);
        let g = gauss_row_ops(&a).unwrap().len();
        let p = pmh_row_ops(&a, default_section_size(n)).unwrap().len();
        assert!(p < g, "pmh {p} vs gauss {g}");
    }
}
