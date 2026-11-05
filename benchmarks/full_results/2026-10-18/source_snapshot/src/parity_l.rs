//! Section V, Theorem 4: Gottesman-Knill is in ⊕L.
//!
//! The proof gives a procedure that finds a determinate measurement outcome
//! using only (a) an oracle for the x- and z-bits of the tableau after any
//! number of gates (these evolve linearly, so a CNOT circuit computes them)
//! and (b) a single phase α ∈ {±1, ±i}, which takes two bits. The nonlinear
//! phase bits r_i are never stored. Instead:
//!
//! 1. α collects the phase of ∏ P_ij^{(T)} over the stabilizers that rowsum
//!    would combine, column by column. That is what r_{2n+1} would be if the
//!    gates never touched any r_i.
//! 2. For each such stabilizer, α is negated once per gate that would have
//!    flipped its r_i.
//!
//! The measured qubit is 1 iff α = −1 at the end. Here the oracle is a
//! phase-free tableau replayed gate by gate, with snapshots kept, which is
//! enough to show the procedure gives the same answers as Section III.

use crate::circuit::{Circuit, Gate};
use crate::error::{value_err, Result};
use crate::pauli::g;
use crate::rows::Rows;
use crate::tableau::Tableau;

/// x_ij^{(t)} and z_ij^{(t)}: tableau bits after the first t gates.
pub trait Oracle {
    fn x(&mut self, t: usize, i: usize, j: usize) -> bool;
    fn z(&mut self, t: usize, i: usize, j: usize) -> bool;
}

/// Stores the tableau bits after every gate. Phase bits are never read.
pub struct HistoryOracle {
    snapshots: Vec<Rows>,
}

impl HistoryOracle {
    pub fn new(circuit: &Circuit) -> Result<Self> {
        if !circuit.is_unitary() {
            return value_err("the oracle replays unitary gates only");
        }
        let mut t = Tableau::new(circuit.num_qubits().max(1));
        let mut snapshots = vec![t.rows().clone()];
        for gate in circuit.gates() {
            t.apply_unitary(gate)?;
            snapshots.push(t.rows().clone());
        }
        Ok(HistoryOracle { snapshots })
    }
}

impl Oracle for HistoryOracle {
    fn x(&mut self, t: usize, i: usize, j: usize) -> bool {
        self.snapshots[t].x(i, j)
    }
    fn z(&mut self, t: usize, i: usize, j: usize) -> bool {
        self.snapshots[t].z(i, j)
    }
}

/// The outcome of measuring qubit `a` after the unitary `circuit`: `None` if
/// it is random, otherwise the determinate bit.
pub fn determinate_outcome(circuit: &Circuit, a: usize) -> Result<Option<bool>> {
    let mut oracle = HistoryOracle::new(circuit)?;
    determinate_outcome_with(circuit, a, &mut oracle)
}

/// Theorem 4's procedure with a caller-supplied oracle.
pub fn determinate_outcome_with<O: Oracle>(
    circuit: &Circuit,
    a: usize,
    oracle: &mut O,
) -> Result<Option<bool>> {
    if !circuit.is_unitary() {
        return value_err("Theorem 4 assumes measurements are deferred to the end");
    }
    let n = circuit.num_qubits();
    if a >= n {
        return value_err(format!("qubit {a} out of range [0, {n})"));
    }
    let gates = circuit.gates();
    let tt = gates.len();

    // Random iff some stabilizer has x_ia = 1.
    if (n..2 * n).any(|i| oracle.x(tt, i, a)) {
        return Ok(None);
    }

    // α = i^alpha.
    let mut alpha: i64 = 0;

    // First loop: phase of the product, ignoring the r_i.
    for j in 0..n {
        let (mut px, mut pz) = (false, false);
        let mut e = 0i64;
        for i in n..2 * n {
            if oracle.x(tt, i - n, a) {
                let (xi, zi) = (oracle.x(tt, i, j), oracle.z(tt, i, j));
                e += g(xi, zi, px, pz) as i64; // P := P_ij · P
                px ^= xi;
                pz ^= zi;
            }
        }
        alpha += e;
    }

    // Second loop: the sign flips that the gates would have made to each r_i.
    for i in n..2 * n {
        if !oracle.x(tt, i - n, a) {
            continue;
        }
        for (t, gate) in gates.iter().enumerate() {
            let flip = match *gate {
                Gate::H(b) | Gate::P(b) => oracle.x(t, i, b) && oracle.z(t, i, b),
                Gate::Cnot(b, c) => {
                    oracle.x(t, i, b)
                        && oracle.z(t, i, c)
                        && (oracle.x(t, i, c) == oracle.z(t, i, b))
                }
                Gate::Measure(_) => unreachable!("checked unitary above"),
            };
            if flip {
                alpha += 2;
            }
        }
    }

    match alpha.rem_euclid(4) {
        0 => Ok(Some(false)),
        2 => Ok(Some(true)),
        _ => value_err("α ended imaginary; the tableau invariants are broken"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment::random_circuit;
    use crate::rng::Rng;

    #[test]
    fn matches_tableau_algorithm() {
        let mut rng = Rng::seed_from_u64(21);
        let mut determinate = 0;
        for trial in 0..200 {
            let n = 1 + trial % 6;
            let beta = 0.3 + (trial % 5) as f64 * 0.3;
            let full = random_circuit(n, beta, &mut rng).unwrap();
            let mut u = Circuit::new(n);
            for gate in full.gates().iter().filter(|g| g.is_unitary()) {
                u.push(*gate).unwrap();
            }
            let mut t = u.tableau().unwrap();
            for a in 0..n {
                let want = t.peek(a).unwrap();
                assert_eq!(determinate_outcome(&u, a).unwrap(), want);
                determinate += want.is_some() as usize;
            }
        }
        assert!(determinate > 50);
    }
}
