//! Linear algebra over GF(2), plus symplectic completion of a stabilizer
//! group to a full tableau.
//!
//! The canonical form (Section VI), the inner product (Section III) and the mixed-state
//! tableau (Section VII A) all need Gaussian elimination. The completion routine
//! builds destabilizers and logical operators for a given set of stabilizers.

use crate::error::{value_err, Result};
use crate::pauli::{num_words, PauliString};

/// Dense bit matrix, row-major, 64 columns per word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitMatrix {
    rows: usize,
    cols: usize,
    w: usize,
    data: Vec<u64>,
}

impl BitMatrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let w = num_words(cols).max(1);
        BitMatrix {
            rows,
            cols,
            w,
            data: vec![0; rows * w],
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, true);
        }
        m
    }

    /// From nested rows of 0/1 values. All rows must have the same length.
    pub fn from_rows(rows: &[Vec<u8>]) -> Result<Self> {
        let cols = rows.first().map_or(0, |r| r.len());
        let mut m = Self::zeros(rows.len(), cols);
        for (i, row) in rows.iter().enumerate() {
            if row.len() != cols {
                return value_err("matrix rows have different lengths");
            }
            for (j, &v) in row.iter().enumerate() {
                m.set(i, j, v != 0);
            }
        }
        Ok(m)
    }

    pub fn to_rows(&self) -> Vec<Vec<u8>> {
        (0..self.rows)
            .map(|i| (0..self.cols).map(|j| self.get(i, j) as u8).collect())
            .collect()
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> bool {
        self.data[i * self.w + j / 64] >> (j % 64) & 1 == 1
    }

    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: bool) {
        let idx = i * self.w + j / 64;
        let m = 1u64 << (j % 64);
        if v {
            self.data[idx] |= m;
        } else {
            self.data[idx] &= !m;
        }
    }

    pub fn row_words(&self, i: usize) -> &[u64] {
        &self.data[i * self.w..(i + 1) * self.w]
    }

    /// row[dst] ^= row[src]
    pub fn row_xor(&mut self, dst: usize, src: usize) {
        for k in 0..self.w {
            let v = self.data[src * self.w + k];
            self.data[dst * self.w + k] ^= v;
        }
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        for k in 0..self.w {
            self.data.swap(a * self.w + k, b * self.w + k);
        }
    }

    pub fn is_zero(&self) -> bool {
        self.data.iter().all(|&w| w == 0)
    }

    pub fn is_identity(&self) -> bool {
        self.rows == self.cols && *self == Self::identity(self.rows)
    }

    pub fn is_symmetric(&self) -> bool {
        self.rows == self.cols && *self == self.transpose()
    }

    pub fn transpose(&self) -> Self {
        let mut t = Self::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.get(i, j) {
                    t.set(j, i, true);
                }
            }
        }
        t
    }

    /// self · other
    pub fn mul(&self, other: &BitMatrix) -> BitMatrix {
        assert_eq!(self.cols, other.rows, "dimension mismatch");
        let mut out = Self::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.get(i, j) {
                    for k in 0..out.w {
                        out.data[i * out.w + k] ^= other.data[j * other.w + k];
                    }
                }
            }
        }
        out
    }

    /// self · v
    pub fn mul_vec(&self, v: &[bool]) -> Vec<bool> {
        (0..self.rows)
            .map(|i| (0..self.cols).filter(|&j| self.get(i, j) && v[j]).count() % 2 == 1)
            .collect()
    }

    /// Gauss–Jordan elimination in place, choosing pivots only in columns
    /// `< max_col`. Pivot rows end up at the top in order. Returns the pivot
    /// columns.
    pub fn rref_limited(&mut self, max_col: usize) -> Vec<usize> {
        let mut pivots = Vec::new();
        let mut row = 0;
        for col in 0..max_col.min(self.cols) {
            if row == self.rows {
                break;
            }
            let Some(p) = (row..self.rows).find(|&r| self.get(r, col)) else {
                continue;
            };
            self.swap_rows(row, p);
            for r in 0..self.rows {
                if r != row && self.get(r, col) {
                    self.row_xor(r, row);
                }
            }
            pivots.push(col);
            row += 1;
        }
        pivots
    }

    pub fn rref(&mut self) -> Vec<usize> {
        self.rref_limited(self.cols)
    }

    pub fn rank(&self) -> usize {
        self.clone().rref().len()
    }

    /// Horizontal concatenation [self | other].
    pub fn hstack(&self, other: &BitMatrix) -> BitMatrix {
        assert_eq!(self.rows, other.rows);
        let mut out = Self::zeros(self.rows, self.cols + other.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                out.set(i, j, self.get(i, j));
            }
            for j in 0..other.cols {
                out.set(i, self.cols + j, other.get(i, j));
            }
        }
        out
    }

    /// Columns `start..end` as a new matrix.
    pub fn columns(&self, start: usize, end: usize) -> BitMatrix {
        let mut out = Self::zeros(self.rows, end - start);
        for i in 0..self.rows {
            for j in start..end {
                out.set(i, j - start, self.get(i, j));
            }
        }
        out
    }

    pub fn inverse(&self) -> Option<BitMatrix> {
        if self.rows != self.cols {
            return None;
        }
        let n = self.rows;
        let mut aug = self.hstack(&Self::identity(n));
        if aug.rref_limited(n).len() < n {
            return None;
        }
        Some(aug.columns(n, 2 * n))
    }

    /// Some x with self · x = b, if one exists.
    pub fn solve(&self, b: &[bool]) -> Option<Vec<bool>> {
        let mut rhs = Self::zeros(self.rows, 1);
        for (i, &v) in b.iter().enumerate() {
            rhs.set(i, 0, v);
        }
        let mut aug = self.hstack(&rhs);
        let pivots = aug.rref_limited(self.cols);
        let mut x = vec![false; self.cols];
        for (r, &c) in pivots.iter().enumerate() {
            x[c] = aug.get(r, self.cols);
        }
        // Rows below the pivots must have a zero right-hand side.
        if (pivots.len()..self.rows).any(|r| aug.get(r, self.cols)) {
            return None;
        }
        Some(x)
    }
}

/// Symplectic vectors of Paulis as rows of a bit matrix: columns 0..n are
/// x-bits, n..2n are z-bits. Phases are dropped.
pub fn paulis_to_matrix(n: usize, paulis: &[PauliString]) -> BitMatrix {
    let mut m = BitMatrix::zeros(paulis.len(), 2 * n);
    for (i, p) in paulis.iter().enumerate() {
        for q in 0..n {
            let (x, z) = p.get(q);
            m.set(i, q, x);
            m.set(i, n + q, z);
        }
    }
    m
}

/// Checks that `stabs` are Hermitian, mutually commuting and independent.
pub fn check_stabilizer_set(n: usize, stabs: &[PauliString]) -> Result<()> {
    for s in stabs {
        if s.num_qubits() != n {
            return value_err(format!("{s} does not act on {n} qubits"));
        }
        if !s.is_hermitian() {
            return value_err(format!("{s} is not Hermitian (phase ±i)"));
        }
    }
    for i in 0..stabs.len() {
        for j in 0..i {
            if stabs[i].anticommutes(&stabs[j]) {
                return value_err(format!("{} and {} anticommute", stabs[j], stabs[i]));
            }
        }
    }
    if paulis_to_matrix(n, stabs).rank() < stabs.len() {
        return value_err("stabilizer generators are not independent");
    }
    Ok(())
}

/// Completes r independent, commuting stabilizers on n qubits to a full
/// symplectic basis. Returns (destabilizers D_0..D_{r−1}, logical X̄_0..,
/// logical Z̄_0..), with n − r logical pairs, such that
///
/// * D_i anticommutes with S_i and commutes with every other S_j,
/// * the D_i commute with each other,
/// * X̄_k anticommutes with Z̄_k only, and commutes with all S and D.
///
/// This is the "solve a set of linear equations" step of Section VII A, in O(n³)
/// time. Returned operators have phase +1.
pub fn symplectic_completion(
    n: usize,
    stabs: &[PauliString],
) -> Result<(Vec<PauliString>, Vec<PauliString>, Vec<PauliString>)> {
    check_stabilizer_set(n, stabs)?;
    let r = stabs.len();
    if r > n {
        return value_err(format!("{r} stabilizers on {n} qubits is too many"));
    }

    // Step A: dual vectors. Row j of W is S_j with its halves swapped, so
    // W · v is the vector of symplectic products ⟨v, S_j⟩. Solve W v = e_i.
    let mut w = BitMatrix::zeros(r, 2 * n);
    for (j, s) in stabs.iter().enumerate() {
        for q in 0..n {
            let (x, z) = s.get(q);
            w.set(j, q, z);
            w.set(j, n + q, x);
        }
    }
    let mut aug = w.hstack(&BitMatrix::identity(r));
    let pivots = aug.rref_limited(2 * n);
    debug_assert_eq!(pivots.len(), r);
    let mut destabs: Vec<PauliString> = (0..r)
        .map(|i| {
            let mut d = PauliString::identity(n);
            for (k, &c) in pivots.iter().enumerate() {
                if aug.get(k, 2 * n + i) {
                    let q = c % n;
                    let (x, z) = d.get(q);
                    if c < n {
                        d.set(q, true, z);
                    } else {
                        d.set(q, x, true);
                    }
                }
            }
            d
        })
        .collect();

    // Step B: make the destabilizers commute. Adding S_j to D_i fixes
    // ⟨D_i, D_j⟩ without touching any ⟨D_i, S_k⟩ or earlier pairs.
    for i in 0..r {
        for j in 0..i {
            if destabs[i].anticommutes(&destabs[j]) {
                destabs[i] = destabs[i].mul(&stabs[j]).without_phase();
            }
        }
    }

    // Step C: project every single-qubit X_q, Z_q onto the symplectic
    // complement of span(S, D), then symplectic Gram–Schmidt into pairs.
    let mut pool: Vec<PauliString> = Vec::with_capacity(2 * n);
    for q in 0..n {
        for letter in ['X', 'Z'] {
            let v = PauliString::single(n, q, letter)?;
            let mut out = v.clone();
            for i in 0..r {
                if v.anticommutes(&stabs[i]) {
                    out = out.mul(&destabs[i]);
                }
                if v.anticommutes(&destabs[i]) {
                    out = out.mul(&stabs[i]);
                }
            }
            if !out.is_identity_letters() {
                pool.push(out.without_phase());
            }
        }
    }
    let mut xbars = Vec::new();
    let mut zbars = Vec::new();
    while xbars.len() < n - r {
        let Some(w1) = pool.pop() else {
            return value_err("symplectic completion failed: ran out of vectors");
        };
        let Some(pos) = pool.iter().position(|u| u.anticommutes(&w1)) else {
            continue;
        };
        let w2 = pool.swap_remove(pos);
        pool = pool
            .into_iter()
            .map(|u| {
                let mut out = u.clone();
                if u.anticommutes(&w2) {
                    out = out.mul(&w1);
                }
                if u.anticommutes(&w1) {
                    out = out.mul(&w2);
                }
                out.without_phase()
            })
            .filter(|u| !u.is_identity_letters())
            .collect();
        xbars.push(w1);
        zbars.push(w2);
    }
    Ok((destabs, xbars, zbars))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_and_solve() {
        let a = BitMatrix::from_rows(&[vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 1]]).unwrap();
        let inv = a.inverse().unwrap();
        assert!(a.mul(&inv).is_identity());
        let b = [true, false, true];
        let x = a.solve(&b).unwrap();
        assert_eq!(a.mul_vec(&x), b);
        let singular = BitMatrix::from_rows(&[vec![1, 1], vec![1, 1]]).unwrap();
        assert!(singular.inverse().is_none());
        assert!(singular.solve(&[true, false]).is_none());
    }

    #[test]
    fn completion_is_symplectic_basis() {
        let n = 5;
        let stabs: Vec<PauliString> = ["XXIII", "ZZIII", "IIYYI"]
            .iter()
            .map(|s| PauliString::parse(s).unwrap())
            .collect();
        let (d, xb, zb) = symplectic_completion(n, &stabs).unwrap();
        assert_eq!((d.len(), xb.len(), zb.len()), (3, 2, 2));
        let mut all = Vec::new();
        all.extend(d.iter().cloned());
        all.extend(xb.iter().cloned());
        all.extend(stabs.iter().cloned());
        all.extend(zb.iter().cloned());
        // Rows i and i+n are partners; every other pair commutes.
        for i in 0..2 * n {
            for j in 0..2 * n {
                let partner = j == (i + n) % (2 * n);
                assert_eq!(all[i].anticommutes(&all[j]), partner, "{i} {j}");
            }
        }
    }
}
