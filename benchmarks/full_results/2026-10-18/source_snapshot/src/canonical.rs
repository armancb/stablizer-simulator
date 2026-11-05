//! Section VI: the canonical form theorem.
//!
//! Theorem 8: every unitary stabilizer circuit is equivalent to one made of
//! 11 rounds in the order H-C-P-C-P-C-H-P-C-P-C (Hadamards, CNOTs, phases).
//! The proof is constructive. Steps (1)–(11) apply gates that take an
//! arbitrary tableau, blocks
//!
//! ```text
//!     ( A | B )    A, B: destabilizer x and z bits
//!     ( C | D )    C, D: stabilizer x and z bits
//! ```
//!
//! to the standard initial tableau. This module follows those steps exactly,
//! with Lemma 6 (Hadamards that make C full rank) and Lemma 7 (a symmetric A
//! plus a diagonal Λ equals M Mᵀ).
//!
//! Convention: a CNOT from c to t adds column c to column t of every x-block
//! (X ← X·E) and column t to column c of every z-block (Z ← Z·Eᵀ). A CNOT
//! round that realizes X ← X·L therefore realizes Z ← Z·L⁻ᵀ.

use crate::circuit::{Circuit, Gate};
use crate::error::{value_err, Error, Result};
use crate::gf2::BitMatrix;
use crate::pmh::{default_section_size, gauss_row_ops, pmh_row_ops};
use crate::tableau::Tableau;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundKind {
    H,
    C,
    P,
}

impl RoundKind {
    pub fn letter(&self) -> char {
        match self {
            RoundKind::H => 'H',
            RoundKind::C => 'C',
            RoundKind::P => 'P',
        }
    }
}

/// H-C-P-C-P-C-H-P-C-P-C
pub const CANONICAL_SEQUENCE: [RoundKind; 11] = {
    use RoundKind::*;
    [H, C, P, C, P, C, H, P, C, P, C]
};

/// How each CNOT round is synthesized.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CnotSynthesis {
    /// Gauss–Jordan elimination, O(n²) gates.
    #[default]
    Gauss,
    /// Patel–Markov–Hayes, O(n²/log n) gates (Corollary 9).
    Pmh,
}

/// A circuit organized into rounds of a single gate kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoundCircuit {
    n: usize,
    rounds: Vec<(RoundKind, Vec<Gate>)>,
}

impl RoundCircuit {
    pub fn num_qubits(&self) -> usize {
        self.n
    }

    pub fn rounds(&self) -> &[(RoundKind, Vec<Gate>)] {
        &self.rounds
    }

    pub fn to_circuit(&self) -> Circuit {
        let mut c = Circuit::new(self.n);
        for (_, gates) in &self.rounds {
            for g in gates {
                c.push(*g).expect("gates were validated when built");
            }
        }
        c
    }

    pub fn gate_count(&self) -> usize {
        self.rounds.iter().map(|(_, g)| g.len()).sum()
    }

    /// The inverse circuit: rounds and gates reversed, with P⁻¹ = P³.
    pub fn inverse(&self) -> RoundCircuit {
        let rounds = self
            .rounds
            .iter()
            .rev()
            .map(|(kind, gates)| {
                let mut inv = Vec::new();
                for g in gates.iter().rev() {
                    let copies = if matches!(g, Gate::P(_)) { 3 } else { 1 };
                    inv.extend(std::iter::repeat_n(*g, copies));
                }
                (*kind, inv)
            })
            .collect();
        RoundCircuit { n: self.n, rounds }
    }
}

/// Lemma 7: for symmetric A, returns (Λ, M) with M lower triangular, unit
/// diagonal (so invertible), and A + Λ = M Mᵀ for the diagonal matrix Λ.
pub fn lemma7(a: &BitMatrix) -> Result<(Vec<bool>, BitMatrix)> {
    if !a.is_symmetric() {
        return value_err("Lemma 7 needs a symmetric matrix");
    }
    let n = a.rows();
    let mut m = BitMatrix::identity(n);
    for i in 0..n {
        for j in 0..i {
            // Equation (4): A_ij = Σ_{k<j} M_ik M_jk + M_ij.
            let mut v = a.get(i, j);
            for k in 0..j {
                v ^= m.get(i, k) & m.get(j, k);
            }
            m.set(i, j, v);
        }
    }
    let lambda = (0..n)
        .map(|i| a.get(i, i) ^ ((0..=i).filter(|&k| m.get(i, k)).count() % 2 == 1))
        .collect();
    Ok((lambda, m))
}

/// Lemma 6: qubits on which Hadamards make the stabilizer x-block C full rank.
pub fn lemma6_hadamards(t: &Tableau) -> Vec<usize> {
    let n = t.num_qubits();
    let rows = t.rows();
    let mut s = BitMatrix::zeros(n, 2 * n);
    for i in 0..n {
        for q in 0..n {
            s.set(i, q, rows.x(n + i, q));
            s.set(i, n + q, rows.z(n + i, q));
        }
    }
    // Row-reduce on the x-block: ( A B ; 0 C ) with A of rank k.
    let k = s.rref_limited(n).len();
    let mut c = BitMatrix::zeros(n - k, n);
    for i in k..n {
        for q in 0..n {
            c.set(i - k, q, s.get(i, n + q));
        }
    }
    // C has rank n − k; Hadamards on its pivot columns finish the job.
    c.rref()
}

/// CNOTs (control, target), in order, realizing X ← X·L on every x-block.
pub fn synth_linear(l: &BitMatrix, method: CnotSynthesis) -> Result<Vec<(usize, usize)>> {
    // Row ops R_1..R_k reduce Lᵀ to I, so Lᵀ = R_1⋯R_k and L = R_kᵀ⋯R_1ᵀ.
    // The row op "row t ⊕= row c" is the transpose of the CNOT(c→t) column
    // op, so the CNOTs are the row ops in reverse order.
    let lt = l.transpose();
    let ops = match method {
        CnotSynthesis::Gauss => gauss_row_ops(&lt)?,
        CnotSynthesis::Pmh => pmh_row_ops(&lt, default_section_size(l.rows()))?,
    };
    Ok(ops.into_iter().rev().collect())
}

fn signs_of(t: &Tableau, stab: bool) -> Vec<bool> {
    let n = t.num_qubits();
    let off = if stab { n } else { 0 };
    (0..n).map(|i| t.rows().r(off + i)).collect()
}

struct Builder {
    t: Tableau,
    rounds: Vec<(RoundKind, Vec<Gate>)>,
}

impl Builder {
    fn round(&mut self, kind: RoundKind, gates: Vec<Gate>) {
        debug_assert_eq!(kind, CANONICAL_SEQUENCE[self.rounds.len()]);
        for g in &gates {
            self.t.apply_unitary(g).expect("in-range gate");
        }
        self.rounds.push((kind, gates));
    }

    fn cnot_round(&mut self, l: &BitMatrix, method: CnotSynthesis) -> Result<()> {
        let gates = synth_linear(l, method)?
            .into_iter()
            .map(|(c, t)| Gate::Cnot(c, t))
            .collect();
        self.round(RoundKind::C, gates);
        Ok(())
    }

    /// One n × n block: rows 0..n (destabilizers) or n..2n, x or z bits.
    fn block(&self, stab: bool, xbits: bool) -> BitMatrix {
        let n = self.t.num_qubits();
        let off = if stab { n } else { 0 };
        let rows = self.t.rows();
        let mut m = BitMatrix::zeros(n, n);
        for i in 0..n {
            for q in 0..n {
                let v = if xbits {
                    rows.x(off + i, q)
                } else {
                    rows.z(off + i, q)
                };
                m.set(i, q, v);
            }
        }
        m
    }

    fn signs(&self, stab: bool) -> Vec<bool> {
        signs_of(&self.t, stab)
    }

    /// Signs as they would be after applying `gates` (which may flip them).
    fn signs_after(&self, gates: &[Gate], stab: bool) -> Vec<bool> {
        let mut t = self.t.clone();
        for g in gates {
            t.apply_unitary(g).expect("in-range gate");
        }
        signs_of(&t, stab)
    }

    fn expect(&self, ok: bool, step: &str) -> Result<()> {
        if ok {
            Ok(())
        } else {
            Err(Error::Value(format!("canonical form: step {step} failed")))
        }
    }
}

/// Gates G in canonical order with G·T = the standard initial tableau
/// (signs included), i.e. G U = I up to global phase, where U is the
/// Clifford that T describes.
pub fn to_identity_circuit(t: &Tableau, method: CnotSynthesis) -> Result<RoundCircuit> {
    t.check_invariants()?;
    let n = t.num_qubits();
    let all = |f: fn(usize) -> Gate| (0..n).map(f).collect::<Vec<_>>();
    let mut b = Builder {
        t: t.clone(),
        rounds: Vec::new(),
    };

    // (1) Hadamards make C full rank (Lemma 6).
    let hs = lemma6_hadamards(&b.t).into_iter().map(Gate::H).collect();
    b.round(RoundKind::H, hs);
    let c = b.block(true, true);
    let c_inv = c.inverse();
    b.expect(c_inv.is_some(), "1 (C full rank)")?;

    // (2) CNOTs: Gaussian elimination on C gives ( A B ; I D ).
    b.cnot_round(&c_inv.unwrap(), method)?;
    b.expect(b.block(true, true).is_identity(), "2 (C = I)")?;

    // (3) D is symmetric; phases add Λ so that D = M Mᵀ (Lemma 7).
    let d = b.block(true, false);
    b.expect(d.is_symmetric(), "3 (D symmetric)")?;
    let (lambda, m) = lemma7(&d)?;
    let ps = (0..n).filter(|&a| lambda[a]).map(Gate::P).collect();
    b.round(RoundKind::P, ps);

    // (4) CNOTs map I to M, and D to M Mᵀ (Mᵀ)⁻¹ = M: ( A B ; M M ).
    b.cnot_round(&m, method)?;
    b.expect(
        b.block(true, true) == m && b.block(true, false) == m,
        "4 (M M)",
    )?;

    // (5) Phases on every qubit give ( A B ; M 0 ). Then two more phases
    // (a Z) on each qubit of v with M v = r clear the stabilizer signs.
    let mut ps = all(Gate::P);
    let v = m
        .solve(&b.signs_after(&ps, true))
        .ok_or_else(|| Error::Value("canonical form: step 5 sign solve failed".into()))?;
    for (a, &on) in v.iter().enumerate() {
        if on {
            ps.extend([Gate::P(a), Gate::P(a)]);
        }
    }
    b.round(RoundKind::P, ps);
    b.expect(
        b.block(true, false).is_zero() && b.signs(true).iter().all(|&s| !s),
        "5 (D = 0, signs +)",
    )?;

    // (6) CNOTs reduce M to I. Commutativity then forces B = I.
    let m_inv = m.inverse().expect("M is unit lower triangular");
    b.cnot_round(&m_inv, method)?;
    b.expect(b.block(true, true).is_identity(), "6 (C = I)")?;
    b.expect(b.block(false, false).is_identity(), "6 (B = I)")?;

    // (7) Hadamards on every qubit give ( I A ; 0 I ).
    b.round(RoundKind::H, all(Gate::H));
    b.expect(
        b.block(false, true).is_identity() && b.block(true, true).is_zero(),
        "7 (I A ; 0 I)",
    )?;

    // (8) A is symmetric; phases make A = N Nᵀ (Lemma 7).
    let a_blk = b.block(false, false);
    b.expect(a_blk.is_symmetric(), "8 (A symmetric)")?;
    let (lambda, nn) = lemma7(&a_blk)?;
    let ps = (0..n).filter(|&a| lambda[a]).map(Gate::P).collect();
    b.round(RoundKind::P, ps);

    // (9) CNOTs give ( N N ; 0 C ).
    b.cnot_round(&nn, method)?;
    b.expect(
        b.block(false, true) == nn && b.block(false, false) == nn,
        "9 (N N)",
    )?;

    // (10) Phases on every qubit give ( N 0 ; 0 C ); Z on each qubit of v
    // with N v = r clears the destabilizer signs.
    let mut ps = all(Gate::P);
    let v = nn
        .solve(&b.signs_after(&ps, false))
        .ok_or_else(|| Error::Value("canonical form: step 10 sign solve failed".into()))?;
    for (a, &on) in v.iter().enumerate() {
        if on {
            ps.extend([Gate::P(a), Gate::P(a)]);
        }
    }
    b.round(RoundKind::P, ps);

    // (11) CNOTs reduce N to I, and C = N⁻ᵀ to I with it.
    let n_inv = nn.inverse().expect("N is unit lower triangular");
    b.cnot_round(&n_inv, method)?;
    b.expect(b.t.is_identity(), "11 (identity tableau)")?;

    Ok(RoundCircuit {
        n,
        rounds: b.rounds,
    })
}

/// A circuit that prepares the Clifford described by `t` from the identity
/// tableau: the inverse of [`to_identity_circuit`], so its rounds come in the
/// reverse order C-P-C-P-H-C-P-C-P-C-H.
pub fn synthesize(t: &Tableau, method: CnotSynthesis) -> Result<RoundCircuit> {
    Ok(to_identity_circuit(t, method)?.inverse())
}

/// Theorem 8: an equivalent circuit in H-C-P-C-P-C-H-P-C-P-C form.
///
/// As footnote 46 explains, the procedure produces the canonical form of the
/// *inverse* of the circuit it is run on. So it is run on the tableau of
/// `circuit⁻¹`: G·T(U⁻¹) = I means G = U.
pub fn canonical_form(circuit: &Circuit, method: CnotSynthesis) -> Result<RoundCircuit> {
    if circuit.num_qubits() == 0 {
        return value_err("circuit has no qubits");
    }
    let t_inv = circuit.inverse()?.tableau()?;
    to_identity_circuit(&t_inv, method)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment::random_circuit;
    use crate::rng::Rng;

    fn random_unitary(n: usize, rng: &mut Rng) -> Circuit {
        let c = random_circuit(n, 3.0, rng).unwrap();
        let mut u = Circuit::new(n);
        for g in c.gates().iter().filter(|g| g.is_unitary()) {
            u.push(*g).unwrap();
        }
        u
    }

    #[test]
    fn lemma7_holds() {
        let mut rng = Rng::seed_from_u64(3);
        for n in 1..12 {
            let mut a = BitMatrix::zeros(n, n);
            for i in 0..n {
                for j in 0..=i {
                    let v = rng.bit();
                    a.set(i, j, v);
                    a.set(j, i, v);
                }
            }
            let (lambda, m) = lemma7(&a).unwrap();
            let mut lhs = a.clone();
            for (i, &l) in lambda.iter().enumerate() {
                lhs.set(i, i, lhs.get(i, i) ^ l);
            }
            assert_eq!(lhs, m.mul(&m.transpose()));
            assert!(m.inverse().is_some());
        }
    }

    #[test]
    fn canonical_form_is_equivalent() {
        let mut rng = Rng::seed_from_u64(11);
        for n in [1, 2, 3, 5, 8, 13] {
            for method in [CnotSynthesis::Gauss, CnotSynthesis::Pmh] {
                let u = random_unitary(n, &mut rng);
                let cf = canonical_form(&u, method).unwrap();
                let kinds: Vec<RoundKind> = cf.rounds().iter().map(|r| r.0).collect();
                assert_eq!(kinds, CANONICAL_SEQUENCE);
                assert_eq!(cf.to_circuit().tableau().unwrap(), u.tableau().unwrap());
            }
        }
    }

    #[test]
    fn synthesize_prepares_tableau() {
        let mut rng = Rng::seed_from_u64(12);
        for n in [1, 4, 9] {
            let t = random_unitary(n, &mut rng).tableau().unwrap();
            let s = synthesize(&t, CnotSynthesis::Gauss).unwrap();
            assert_eq!(s.to_circuit().tableau().unwrap(), t);
        }
    }
}
