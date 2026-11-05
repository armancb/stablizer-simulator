//! Section VII C: circuits with a few non-stabilizer gates.
//!
//! Start from |0…0⟩ and allow unlimited stabilizer gates and Pauli
//! measurements, plus d gates that each act on at most b qubits. The density
//! matrix is kept as
//!
//! ρ = Σ_t c_t · P_t · ρ_{s_t},   ρ_s = 2^{−n} ∏_j (I + (−1)^{s_j} M_j),
//!
//! where M_1..M_n are the stabilizer generators held in a tableau (with
//! destabilizers), P_t is a Pauli and s_t ∈ {0,1}^n flips generator signs.
//!
//! * A stabilizer gate G conjugates each P_t and updates the tableau.
//! * A gate U = Σ_i u_i P_i (its Pauli expansion) maps c P ρ_s to
//!   Σ_{i,k} c u_i ū_k (P_i P P_k) ρ_{s ⊕ σ(P_k)}, where σ_j(P_k) = M_j·P_k is
//!   the symplectic product (ρ_s P_k = P_k ρ_{s⊕σ(P_k)}).
//! * Measuring a Pauli Q is handled by the two cases in the paper; see
//!   [`NonStabilizerSimulator::measure_pauli`].
//!
//! Terms with the same (P, s) are merged. After d gates there are at most
//! 4^{2bd} terms, so the cost is O(4^{2bd} n + n²) per operation.

use crate::error::{index_err, value_err, Result};
use crate::nonstab_init::pauli_trace;
use crate::pauli::{num_words, PauliString};
use crate::rng::Rng;
use crate::tableau::Tableau;
use num_complex::Complex64;
use std::collections::HashMap;

/// Coefficients smaller than this are dropped.
const PRUNE: f64 = 1e-14;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    /// Letters of P (phase folded into the coefficient).
    x: Vec<u64>,
    z: Vec<u64>,
    /// Sign flips s, one bit per stabilizer generator.
    s: Vec<u64>,
}

#[derive(Clone, Debug)]
struct State {
    tab: Tableau,
    terms: HashMap<Key, Complex64>,
}

#[derive(Clone, Debug)]
pub struct NonStabilizerSimulator {
    n: usize,
    state: State,
}

fn ipow(k: u8) -> Complex64 {
    match k % 4 {
        0 => Complex64::new(1.0, 0.0),
        1 => Complex64::new(0.0, 1.0),
        2 => Complex64::new(-1.0, 0.0),
        _ => Complex64::new(0.0, -1.0),
    }
}

#[inline]
fn bit(s: &[u64], j: usize) -> bool {
    s[j / 64] >> (j % 64) & 1 == 1
}

#[inline]
fn flip(s: &mut [u64], j: usize) {
    s[j / 64] ^= 1 << (j % 64);
}

impl State {
    fn key_pauli(&self, key: &Key) -> PauliString {
        PauliString::from_words(self.tab.num_qubits(), key.x.clone(), key.z.clone(), 0)
    }

    fn add(terms: &mut HashMap<Key, Complex64>, p: &PauliString, s: Vec<u64>, c: Complex64) {
        let c = c * ipow(p.phase());
        let key = Key {
            x: p.x_words().to_vec(),
            z: p.z_words().to_vec(),
            s,
        };
        *terms.entry(key).or_insert(Complex64::new(0.0, 0.0)) += c;
    }

    fn prune(&mut self) {
        self.terms.retain(|_, c| c.norm() > PRUNE);
    }

    /// σ(P): bit j is set iff P anticommutes with generator M_j.
    fn sigma(&self, p: &PauliString) -> Vec<u64> {
        let n = self.tab.num_qubits();
        let mut s = vec![0u64; num_words(n)];
        for j in 0..n {
            if self.tab.rows().anticommutes_with(n + j, p) {
                flip(&mut s, j);
            }
        }
        s
    }

    /// If P commutes with every generator, P = φ ∏_{j∈J} M_j with
    /// J = {j : P anticommutes with D_j}; returns (J, φ ∈ {±1}) for a
    /// Hermitian P, or None if P is not in the stabilizer group up to phase.
    fn decompose(&self, p: &PauliString) -> Option<(Vec<usize>, Complex64)> {
        let n = self.tab.num_qubits();
        let rows = self.tab.rows();
        if (0..n).any(|j| rows.anticommutes_with(n + j, p)) {
            return None;
        }
        let js: Vec<usize> = (0..n).filter(|&j| rows.anticommutes_with(j, p)).collect();
        let mut prod = PauliString::identity(n);
        for &j in &js {
            prod = prod.mul(&rows.row_pauli(n + j));
        }
        debug_assert_eq!(prod.x_words(), p.x_words());
        debug_assert_eq!(prod.z_words(), p.z_words());
        // p = i^{p.phase} L and prod = i^{prod.phase} L, so p = φ · prod.
        let phi = ipow((4 + p.phase() - prod.phase()) % 4);
        Some((js, phi))
    }

    /// Tr ρ = Σ_t c_t Tr(P_t ρ_{s_t}). Tr(P ρ_s) is the eigenvalue of P on the
    /// pure state ρ_s if P is (up to phase) in the stabilizer, else 0.
    fn trace(&self) -> Complex64 {
        let mut total = Complex64::new(0.0, 0.0);
        for (key, &c) in &self.terms {
            let p = self.key_pauli(key);
            if let Some((js, phi)) = self.decompose(&p) {
                let negs = js.iter().filter(|&&j| bit(&key.s, j)).count();
                let sign = if negs % 2 == 1 { -1.0 } else { 1.0 };
                total += c * phi * sign;
            }
        }
        total
    }

    fn conjugate_terms(&mut self, f: impl Fn(&mut PauliString)) {
        let old = std::mem::take(&mut self.terms);
        for (key, c) in old {
            let mut p = self.key_pauli(&key);
            f(&mut p);
            State::add(&mut self.terms, &p, key.s, c);
        }
    }

    /// The unnormalized branch Π ρ Π with Π = (I + (−1)^m Q)/2.
    fn project(&self, q: &PauliString, m: bool) -> State {
        let n = self.tab.num_qubits();
        let mut qm = q.clone();
        if m {
            qm.mul_phase(2);
        }
        let mut out = self.clone();
        let anti: Vec<usize> = (0..n)
            .filter(|&j| out.tab.rows().anticommutes_with(n + j, &qm))
            .collect();

        if anti.is_empty() {
            // Q commutes with the whole stabilizer, so qm = λ ∏_{j∈J} M_j.
            // A term survives iff P commutes with Q and qm has eigenvalue
            // +1 on ρ_s.
            let (js, lambda) = out.decompose(&qm).expect("commutes with all generators");
            let old = std::mem::take(&mut out.terms);
            for (key, c) in old {
                let p = out.key_pauli(&key);
                if p.anticommutes(&qm) {
                    continue;
                }
                let negs = js.iter().filter(|&&j| bit(&key.s, j)).count();
                let eig = lambda.re * if negs % 2 == 1 { -1.0 } else { 1.0 };
                if eig > 0.0 {
                    out.terms.insert(key, c);
                }
            }
            return out;
        }

        // Q anticommutes with some generator. Make M_p the only one by
        // M_j ← M_p M_j for the others. That maps sign bits s_j ← s_j ⊕ s_p,
        // and the destabilizers follow with D_p ← D_j D_p.
        let p = anti[0];
        for &j in &anti[1..] {
            let rows = out.tab.rows_mut();
            rows.rowsum(n + j, n + p).expect("generators commute");
            rows.rowsum(p, j).expect("destabilizers commute");
        }
        if anti.len() > 1 {
            let old = std::mem::take(&mut out.terms);
            for (mut key, c) in old {
                if bit(&key.s, p) {
                    for &j in &anti[1..] {
                        flip(&mut key.s, j);
                    }
                }
                *out.terms.entry(key).or_insert(Complex64::new(0.0, 0.0)) += c;
            }
        }
        let mp = out.tab.rows().row_pauli(n + p);

        // Π P (I + ε_p M_p) Λ Π: if P commutes with Q this is P Π Λ; if it
        // anticommutes, ε_p P M_p Π Λ. Here Π Λ = ½ ρ' for the new stabilizer
        // in which Q_m replaces M_p (with s_p = 0).
        let old = std::mem::take(&mut out.terms);
        for (mut key, c) in old {
            let pauli = out.key_pauli(&key);
            let sp = bit(&key.s, p);
            if sp {
                flip(&mut key.s, p);
            }
            if pauli.commutes(&qm) {
                State::add(&mut out.terms, &pauli, key.s, c * 0.5);
            } else {
                let eps = if sp { -0.5 } else { 0.5 };
                State::add(&mut out.terms, &pauli.mul(&mp), key.s, c * eps);
            }
        }

        // New tableau: destabilizers that anticommute with Q absorb M_p, then
        // D_p = old M_p and M_p = Q_m.
        let rows = out.tab.rows_mut();
        for i in 0..n {
            if i != p && rows.anticommutes_with(i, &qm) {
                rows.rowsum(i, n + p).expect("D_i commutes with M_p");
            }
        }
        rows.set_row_pauli(p, &mp);
        rows.set_row_pauli(n + p, &qm);
        out.prune();
        out
    }
}

impl NonStabilizerSimulator {
    /// The state |0…0⟩ on n qubits.
    pub fn new(n: usize) -> Result<Self> {
        if n == 0 {
            return value_err("n must be positive");
        }
        let mut terms = HashMap::new();
        let w = num_words(n);
        terms.insert(
            Key {
                x: vec![0; w],
                z: vec![0; w],
                s: vec![0; w],
            },
            Complex64::new(1.0, 0.0),
        );
        Ok(NonStabilizerSimulator {
            n,
            state: State {
                tab: Tableau::new(n),
                terms,
            },
        })
    }

    pub fn num_qubits(&self) -> usize {
        self.n
    }

    /// Number of (P, s) terms in the decomposition.
    pub fn num_terms(&self) -> usize {
        self.state.terms.len()
    }

    pub fn stabilizers(&self) -> Vec<PauliString> {
        self.state.tab.stabilizers()
    }

    fn check_qubit(&self, q: usize) -> Result<()> {
        if q >= self.n {
            return index_err(format!("qubit {q} out of range [0, {})", self.n));
        }
        Ok(())
    }

    pub fn cnot(&mut self, a: usize, b: usize) -> Result<()> {
        self.state.tab.cnot(a, b)?;
        self.state.conjugate_terms(|p| p.conj_cnot(a, b));
        Ok(())
    }

    pub fn h(&mut self, a: usize) -> Result<()> {
        self.state.tab.h(a)?;
        self.state.conjugate_terms(|p| p.conj_h(a));
        Ok(())
    }

    pub fn p(&mut self, a: usize) -> Result<()> {
        self.state.tab.p(a)?;
        self.state.conjugate_terms(|p| p.conj_p(a));
        Ok(())
    }

    /// Applies an arbitrary unitary on `qubits` (at most 8). `matrix` is
    /// row-major, 2^b × 2^b, with the first listed qubit as the most
    /// significant bit.
    pub fn apply_unitary(&mut self, matrix: &[Complex64], qubits: &[usize]) -> Result<()> {
        let b = qubits.len();
        if b == 0 || b > 8 {
            return value_err("a gate must act on 1 to 8 qubits");
        }
        for (i, &q) in qubits.iter().enumerate() {
            self.check_qubit(q)?;
            if qubits[..i].contains(&q) {
                return value_err(format!("qubit {q} listed twice"));
            }
        }
        let dim = 1usize << b;
        if matrix.len() != dim * dim {
            return value_err(format!("a gate on {b} qubits needs a {dim}×{dim} matrix"));
        }
        for i in 0..dim {
            for j in 0..dim {
                let dot: Complex64 = (0..dim)
                    .map(|k| matrix[i * dim + k] * matrix[j * dim + k].conj())
                    .sum();
                let want = if i == j { 1.0 } else { 0.0 };
                if (dot - want).norm() > 1e-8 {
                    return value_err("matrix is not unitary");
                }
            }
        }

        // Pauli expansion U = Σ u_i P_i with u_i = Tr(P_i U) / 2^b.
        let n = self.n;
        let mut expansion: Vec<(PauliString, Complex64)> = Vec::new();
        for xm in 0..dim {
            for zm in 0..dim {
                let u = pauli_trace(xm, zm, matrix, dim) / dim as f64;
                if u.norm() <= PRUNE {
                    continue;
                }
                let mut p = PauliString::identity(n);
                for (pos, &q) in qubits.iter().enumerate() {
                    let bitpos = b - 1 - pos;
                    p.set(q, xm >> bitpos & 1 == 1, zm >> bitpos & 1 == 1);
                }
                expansion.push((p, u));
            }
        }
        let sigmas: Vec<Vec<u64>> = expansion.iter().map(|(p, _)| self.state.sigma(p)).collect();

        let old = std::mem::take(&mut self.state.terms);
        let mut new = HashMap::new();
        for (key, c) in old {
            let p = self.state.key_pauli(&key);
            for (pi, ui) in &expansion {
                let left = pi.mul(&p);
                for ((pk, uk), sig) in expansion.iter().zip(&sigmas) {
                    let s: Vec<u64> = key.s.iter().zip(sig).map(|(a, b)| a ^ b).collect();
                    State::add(&mut new, &left.mul(pk), s, c * ui * uk.conj());
                }
            }
        }
        self.state.terms = new;
        self.state.prune();
        Ok(())
    }

    /// T = diag(1, e^{iπ/4}).
    pub fn t(&mut self, a: usize) -> Result<()> {
        let e = Complex64::from_polar(1.0, std::f64::consts::FRAC_PI_4);
        let zero = Complex64::new(0.0, 0.0);
        self.apply_unitary(&[Complex64::new(1.0, 0.0), zero, zero, e], &[a])
    }

    fn check_pauli(&self, q: &PauliString) -> Result<()> {
        if q.num_qubits() != self.n {
            return value_err(format!("{q} does not act on {} qubits", self.n));
        }
        if !q.is_hermitian() {
            return value_err(format!("{q} is not Hermitian"));
        }
        if q.is_identity_letters() {
            return value_err("cannot measure the identity");
        }
        Ok(())
    }

    /// Probability that measuring the Pauli `q` gives +1 (outcome 0).
    pub fn probability_pauli(&self, q: &PauliString) -> Result<f64> {
        self.check_pauli(q)?;
        let total = self.state.trace().re;
        let p0 = self.state.project(q, false).trace().re;
        Ok((p0 / total).clamp(0.0, 1.0))
    }

    /// Probability that measuring qubit `a` in the standard basis gives 0.
    pub fn probability(&self, a: usize) -> Result<f64> {
        self.check_qubit(a)?;
        self.probability_pauli(&PauliString::single(self.n, a, 'Z')?)
    }

    fn measure_pauli_with(
        &mut self,
        q: &PauliString,
        choose: impl FnOnce(f64) -> bool,
    ) -> Result<bool> {
        self.check_pauli(q)?;
        let total = self.state.trace().re;
        let b0 = self.state.project(q, false);
        let p0 = (b0.trace().re / total).clamp(0.0, 1.0);
        let outcome = choose(p0);
        let mut chosen = if outcome {
            self.state.project(q, true)
        } else {
            b0
        };
        let t = chosen.trace().re;
        if t <= 1e-12 * total.abs().max(1e-300) {
            return value_err(format!("outcome {} has probability 0", outcome as u8));
        }
        for c in chosen.terms.values_mut() {
            *c /= t;
        }
        self.state = chosen;
        Ok(outcome)
    }

    /// Measures the Hermitian Pauli `q`; returns true for eigenvalue −1.
    pub fn measure_pauli(&mut self, q: &PauliString, rng: &mut Rng) -> Result<bool> {
        self.measure_pauli_with(q, |p0| rng.f64() >= p0)
    }

    pub fn measure_pauli_forced(&mut self, q: &PauliString, outcome: bool) -> Result<bool> {
        self.measure_pauli_with(q, |_| outcome)
    }

    pub fn measure(&mut self, a: usize, rng: &mut Rng) -> Result<bool> {
        self.check_qubit(a)?;
        self.measure_pauli(&PauliString::single(self.n, a, 'Z')?, rng)
    }

    pub fn measure_forced(&mut self, a: usize, outcome: bool) -> Result<bool> {
        self.check_qubit(a)?;
        self.measure_pauli_forced(&PauliString::single(self.n, a, 'Z')?, outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hth_probability() {
        // H T H |0⟩: p(0) = |(1 + e^{iπ/4})/2|² = (1 + cos π/4)/2.
        let mut sim = NonStabilizerSimulator::new(1).unwrap();
        sim.h(0).unwrap();
        sim.t(0).unwrap();
        sim.h(0).unwrap();
        let want = (1.0 + std::f64::consts::FRAC_1_SQRT_2) / 2.0;
        assert!((sim.probability(0).unwrap() - want).abs() < 1e-12);
    }

    #[test]
    fn stabilizer_only_matches_tableau() {
        let mut sim = NonStabilizerSimulator::new(2).unwrap();
        sim.h(0).unwrap();
        sim.cnot(0, 1).unwrap();
        assert!((sim.probability(1).unwrap() - 0.5).abs() < 1e-12);
        sim.measure_forced(0, true).unwrap();
        assert!(sim.probability(1).unwrap().abs() < 1e-12);
        assert_eq!(sim.num_terms(), 1);
    }

    #[test]
    fn t_squared_is_p() {
        // T T = P, so H T T H T T H … equals the stabilizer circuit H P H P H.
        let mut a = NonStabilizerSimulator::new(1).unwrap();
        let mut b = Tableau::new(1);
        for _ in 0..2 {
            a.h(0).unwrap();
            a.t(0).unwrap();
            a.t(0).unwrap();
            b.h(0).unwrap();
            b.p(0).unwrap();
        }
        a.h(0).unwrap();
        b.h(0).unwrap();
        let want = match b.peek(0).unwrap() {
            Some(false) => 1.0,
            Some(true) => 0.0,
            None => 0.5,
        };
        assert!((a.probability(0).unwrap() - want).abs() < 1e-12);
    }
}
