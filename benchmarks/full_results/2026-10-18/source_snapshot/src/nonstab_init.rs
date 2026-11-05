//! Section VII B: stabilizer circuits on non-stabilizer initial states.
//!
//! The initial state ρ = ⊗_j ρ_j is a tensor product of blocks of at most b
//! qubits each, in arbitrary (mixed) states. Stabilizer gates are unlimited,
//! but only d measurements are allowed. Measurement k projects onto
//! (I ± Z_{a_k})/2. Pulled back to time 0 through the gates applied so far,
//! that becomes a projector Π_k = (I ± Q_k)/2 with Q_k = U† Z_{a_k} U, a
//! Pauli operator. So
//!
//! p(m_1 … m_d) = Tr[Π_d ⋯ Π_1 ρ Π_1 ⋯ Π_d] = Tr[Π_d ⋯ Π_1 ρ Π_1 ⋯ Π_{d−1}].
//!
//! The second form uses cyclicity of the trace and Π_d² = Π_d. Expanding
//! leaves 2^{2d−1} terms, each the trace of a Pauli against ρ, which is the
//! product over blocks of Tr(P_j ρ_j). Total time is O(m 2^{2d+b}), within
//! the paper's O(m 2^{2b+2d}) bound.
//!
//! Block matrices use the block's qubit order, with the first listed qubit as
//! the most significant bit (the `numpy.kron` convention).

use crate::circuit::Gate;
use crate::error::{index_err, value_err, Result};
use crate::pauli::PauliString;
use crate::rng::Rng;
use num_complex::Complex64;

/// Maximum number of measurements: the cost grows as 4^d.
pub const MAX_MEASUREMENTS: usize = 12;

#[derive(Clone, Debug)]
struct Block {
    qubits: Vec<usize>,
    dim: usize,
    /// Row-major dim × dim density matrix.
    rho: Vec<Complex64>,
}

#[derive(Clone, Debug)]
pub struct ProductStateSimulator {
    n: usize,
    blocks: Vec<Block>,
    gates: Vec<Gate>,
    measured: Vec<(PauliString, bool)>,
    /// p(m_1 … m_k) for the outcomes so far.
    prob_so_far: f64,
}

/// i^k
fn ipow(k: u32) -> Complex64 {
    match k % 4 {
        0 => Complex64::new(1.0, 0.0),
        1 => Complex64::new(0.0, 1.0),
        2 => Complex64::new(-1.0, 0.0),
        _ => Complex64::new(0.0, -1.0),
    }
}

/// Tr(P M) for the Pauli with local masks (xm, zm) and a dim × dim matrix M.
///
/// P|v⟩ = i^{#Y} (−1)^{|v ∧ zm|} |v ⊕ xm⟩, so only one entry per row is nonzero.
pub(crate) fn pauli_trace(xm: usize, zm: usize, m: &[Complex64], dim: usize) -> Complex64 {
    let mut acc = Complex64::new(0.0, 0.0);
    for v in 0..dim {
        let term = m[v * dim + (v ^ xm)];
        if (v & zm).count_ones() % 2 == 1 {
            acc -= term;
        } else {
            acc += term;
        }
    }
    acc * ipow((xm & zm).count_ones())
}

impl ProductStateSimulator {
    /// `blocks` lists (qubits, density matrix). Qubits not in any block start
    /// in |0⟩. Each matrix must be Hermitian with unit trace.
    pub fn new(n: usize, blocks: Vec<(Vec<usize>, Vec<Complex64>)>) -> Result<Self> {
        if n == 0 {
            return value_err("n must be positive");
        }
        let mut used = vec![false; n];
        let mut out = Vec::new();
        for (qubits, rho) in blocks {
            if qubits.is_empty() || qubits.len() > 16 {
                return value_err("a block must have between 1 and 16 qubits");
            }
            for &q in &qubits {
                if q >= n {
                    return index_err(format!("qubit {q} out of range [0, {n})"));
                }
                if used[q] {
                    return value_err(format!("qubit {q} appears in two blocks"));
                }
                used[q] = true;
            }
            let dim = 1usize << qubits.len();
            if rho.len() != dim * dim {
                return value_err(format!(
                    "block on {} qubits needs a {dim}×{dim} matrix",
                    qubits.len()
                ));
            }
            let trace: Complex64 = (0..dim).map(|i| rho[i * dim + i]).sum();
            if (trace - 1.0).norm() > 1e-8 {
                return value_err(format!("density matrix has trace {trace}, not 1"));
            }
            for i in 0..dim {
                for j in 0..dim {
                    if (rho[i * dim + j] - rho[j * dim + i].conj()).norm() > 1e-8 {
                        return value_err("density matrix is not Hermitian");
                    }
                }
            }
            out.push(Block { qubits, dim, rho });
        }
        for (q, &taken) in used.iter().enumerate() {
            if !taken {
                let mut rho = vec![Complex64::new(0.0, 0.0); 4];
                rho[0] = Complex64::new(1.0, 0.0);
                out.push(Block {
                    qubits: vec![q],
                    dim: 2,
                    rho,
                });
            }
        }
        Ok(ProductStateSimulator {
            n,
            blocks: out,
            gates: Vec::new(),
            measured: Vec::new(),
            prob_so_far: 1.0,
        })
    }

    pub fn num_qubits(&self) -> usize {
        self.n
    }

    pub fn num_measurements(&self) -> usize {
        self.measured.len()
    }

    fn check_qubit(&self, q: usize) -> Result<()> {
        if q >= self.n {
            return index_err(format!("qubit {q} out of range [0, {})", self.n));
        }
        Ok(())
    }

    pub fn cnot(&mut self, a: usize, b: usize) -> Result<()> {
        self.check_qubit(a)?;
        self.check_qubit(b)?;
        if a == b {
            return value_err("CNOT control and target must differ");
        }
        self.gates.push(Gate::Cnot(a, b));
        Ok(())
    }

    pub fn h(&mut self, a: usize) -> Result<()> {
        self.check_qubit(a)?;
        self.gates.push(Gate::H(a));
        Ok(())
    }

    pub fn p(&mut self, a: usize) -> Result<()> {
        self.check_qubit(a)?;
        self.gates.push(Gate::P(a));
        Ok(())
    }

    /// Q = U† Z_a U, with U the gates applied so far.
    fn pull_back(&self, a: usize) -> PauliString {
        let mut q = PauliString::single(self.n, a, 'Z').expect("qubit checked");
        for g in self.gates.iter().rev() {
            q.conj_gate_inverse(g);
        }
        q
    }

    /// Tr(W ρ) = i^phase ∏_blocks Tr(W_j ρ_j).
    fn trace_pauli(&self, w: &PauliString) -> Complex64 {
        let mut acc = ipow(w.phase() as u32);
        for b in &self.blocks {
            let k = b.qubits.len();
            let (mut xm, mut zm) = (0usize, 0usize);
            for (pos, &q) in b.qubits.iter().enumerate() {
                let (x, z) = w.get(q);
                let bit = 1 << (k - 1 - pos);
                if x {
                    xm |= bit;
                }
                if z {
                    zm |= bit;
                }
            }
            if xm == 0 && zm == 0 {
                continue; // Tr(ρ_j) = 1
            }
            acc *= pauli_trace(xm, zm, &b.rho, b.dim);
            if acc.norm() == 0.0 {
                break;
            }
        }
        acc
    }

    /// p(m_1 … m_d) for a sequence of pulled-back measurements.
    fn sequence_probability(&self, seq: &[(PauliString, bool)]) -> f64 {
        let d = seq.len();
        if d == 0 {
            return 1.0;
        }
        let signed: Vec<PauliString> = seq
            .iter()
            .map(|(q, m)| {
                let mut q = q.clone();
                if *m {
                    q.mul_phase(2);
                }
                q
            })
            .collect();
        let id = PauliString::identity(self.n);
        // A_L = ∏_{k ∈ L, descending} ε_k Q_k over subsets L of {0..d}.
        let lefts: Vec<PauliString> = (0..1usize << d)
            .map(|mask| {
                (0..d)
                    .rev()
                    .filter(|k| mask >> k & 1 == 1)
                    .fold(id.clone(), |acc, k| acc.mul(&signed[k]))
            })
            .collect();
        // B_R = ∏_{k ∈ R, ascending} ε_k Q_k over subsets R of {0..d−1}.
        let rights: Vec<PauliString> = (0..1usize << (d - 1))
            .map(|mask| {
                (0..d - 1)
                    .filter(|k| mask >> k & 1 == 1)
                    .fold(id.clone(), |acc, k| acc.mul(&signed[k]))
            })
            .collect();
        let mut total = Complex64::new(0.0, 0.0);
        for b in &rights {
            for a in &lefts {
                // Tr[A ρ B] = Tr[B A ρ]
                total += self.trace_pauli(&b.mul(a));
            }
        }
        total.re / (1u64 << (2 * d - 1)) as f64
    }

    /// Probability that measuring qubit `a` now gives 0.
    pub fn probability(&self, a: usize) -> Result<f64> {
        self.check_qubit(a)?;
        if self.measured.len() >= MAX_MEASUREMENTS {
            return value_err(format!(
                "at most {MAX_MEASUREMENTS} measurements are supported"
            ));
        }
        let mut seq = self.measured.clone();
        seq.push((self.pull_back(a), false));
        Ok((self.sequence_probability(&seq) / self.prob_so_far).clamp(0.0, 1.0))
    }

    fn record(&mut self, a: usize, outcome: bool) -> Result<()> {
        let mut seq = self.measured.clone();
        seq.push((self.pull_back(a), outcome));
        let p = self.sequence_probability(&seq);
        if p <= 1e-12 * self.prob_so_far {
            return value_err(format!("outcome {} has probability 0", outcome as u8));
        }
        self.measured = seq;
        self.prob_so_far = p;
        Ok(())
    }

    /// Measures qubit `a`, sampling the outcome (true = 1).
    pub fn measure(&mut self, a: usize, rng: &mut Rng) -> Result<bool> {
        let p0 = self.probability(a)?;
        let outcome = rng.f64() >= p0;
        self.record(a, outcome)?;
        Ok(outcome)
    }

    /// Post-selects outcome `outcome`; returns its probability.
    pub fn measure_forced(&mut self, a: usize, outcome: bool) -> Result<f64> {
        let p0 = self.probability(a)?;
        self.record(a, outcome)?;
        Ok(if outcome { 1.0 - p0 } else { p0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(re: f64, im: f64) -> Complex64 {
        Complex64::new(re, im)
    }

    #[test]
    fn plus_state_measures_evenly() {
        // |+⟩⟨+| = ½[[1,1],[1,1]]
        let plus = vec![c(0.5, 0.0); 4];
        let sim = ProductStateSimulator::new(1, vec![(vec![0], plus.clone())]).unwrap();
        assert!((sim.probability(0).unwrap() - 0.5).abs() < 1e-12);
        let mut sim = ProductStateSimulator::new(1, vec![(vec![0], plus)]).unwrap();
        sim.h(0).unwrap();
        assert!((sim.probability(0).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn magic_state_probabilities() {
        // T|+⟩: ρ = ½[[1, e^{−iπ/4}], [e^{iπ/4}, 1]]. After H,
        // p(0) = (1 + cos π/4)/2.
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let rho = vec![
            c(0.5, 0.0),
            c(0.5 * s, -0.5 * s),
            c(0.5 * s, 0.5 * s),
            c(0.5, 0.0),
        ];
        let mut sim = ProductStateSimulator::new(1, vec![(vec![0], rho)]).unwrap();
        sim.h(0).unwrap();
        assert!((sim.probability(0).unwrap() - (1.0 + s) / 2.0).abs() < 1e-12);
    }

    #[test]
    fn repeated_measurement_is_consistent() {
        let rho = vec![c(0.7, 0.0), c(0.1, 0.2), c(0.1, -0.2), c(0.3, 0.0)];
        let mut sim = ProductStateSimulator::new(2, vec![(vec![0], rho)]).unwrap();
        sim.cnot(0, 1).unwrap();
        let p = sim.measure_forced(0, true).unwrap();
        assert!((p - 0.3).abs() < 1e-12);
        assert!((sim.probability(1).unwrap() - 0.0).abs() < 1e-12);
        assert!(sim.measure_forced(1, false).is_err());
    }
}
