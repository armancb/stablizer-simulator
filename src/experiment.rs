//! The experiment of Section IV (Fig. 2): ⌊βn log₂ n⌋ random unitary gates, each a
//! CNOT, Hadamard or phase with probability 1/3 on uniformly random qubits
//! (a ≠ b), followed by a measurement of every qubit in sequence. Only the
//! measurements are timed.

use crate::circuit::{Circuit, Gate};
use crate::error::{value_err, Result};
use crate::rng::Rng;
use crate::tableau::Tableau;
use std::time::Instant;

/// The random circuit for one data point of Fig. 2, measurements included.
pub fn random_circuit(n: usize, beta: f64, rng: &mut Rng) -> Result<Circuit> {
    if n == 0 {
        return value_err("n must be positive");
    }
    if beta.is_nan() || beta < 0.0 {
        return value_err("beta must be non-negative");
    }
    let count = (beta * n as f64 * (n as f64).log2()).floor() as usize;
    let mut c = Circuit::new(n);
    for _ in 0..count {
        let a = rng.below(n);
        let kind = if n == 1 {
            1 + rng.below(2)
        } else {
            rng.below(3)
        };
        let gate = match kind {
            0 => {
                let mut b = rng.below(n - 1);
                if b >= a {
                    b += 1;
                }
                Gate::Cnot(a, b)
            }
            1 => Gate::H(a),
            _ => Gate::P(a),
        };
        c.push(gate)?;
    }
    for a in 0..n {
        c.measure(a)?;
    }
    Ok(c)
}

/// Seconds per measurement for one random circuit; gate time is excluded.
pub fn time_measurements(n: usize, beta: f64, rng: &mut Rng) -> Result<f64> {
    let circuit = random_circuit(n, beta, rng)?;
    let mut tab = Tableau::new(n);
    let mut measured = Vec::new();
    for g in circuit.gates() {
        match *g {
            Gate::Measure(a) => measured.push(a),
            _ => tab.apply_unitary(g)?,
        }
    }
    let start = Instant::now();
    for &a in &measured {
        tab.measure(a, rng)?;
    }
    Ok(start.elapsed().as_secs_f64() / measured.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_count_follows_beta() {
        let mut rng = Rng::seed_from_u64(0);
        let c = random_circuit(64, 1.0, &mut rng).unwrap();
        assert_eq!(c.len(), 64 * 6 + 64);
        assert!(time_measurements(32, 0.8, &mut rng).unwrap() >= 0.0);
    }
}
