//! The CHP "quantum assembly language" of Section IV.
//!
//! Instructions are `c a b` (CNOT), `h a`, `p a` and `m a`. As in the
//! original CHP, any text before a line beginning with `#` is a free-form
//! description and is ignored. The number of qubits is one more than the
//! largest index used.

use crate::circuit::{Circuit, Gate};
use crate::error::{value_err, Result};
use crate::rng::Rng;
use crate::tableau::Tableau;

pub fn parse(text: &str) -> Result<Circuit> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim_start().starts_with('#'))
        .map_or(0, |i| i + 1);

    let mut gates = Vec::new();
    let mut max_q = 0usize;
    for (offset, line) in lines[start..].iter().enumerate() {
        let lineno = start + offset + 1;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        let arg = |i: usize| -> Result<usize> {
            match tokens.get(i).map(|t| t.parse::<usize>()) {
                Some(Ok(v)) => Ok(v),
                _ => value_err(format!("line {lineno}: expected a qubit index in {line:?}")),
            }
        };
        let (gate, arity) = match tokens[0].to_ascii_lowercase().as_str() {
            "c" => (Gate::Cnot(arg(1)?, arg(2)?), 3),
            "h" => (Gate::H(arg(1)?), 2),
            "p" => (Gate::P(arg(1)?), 2),
            "m" => (Gate::Measure(arg(1)?), 2),
            other => return value_err(format!("line {lineno}: unknown instruction {other:?}")),
        };
        if tokens.len() != arity {
            return value_err(format!(
                "line {lineno}: wrong number of arguments in {line:?}"
            ));
        }
        if let Gate::Cnot(a, b) = gate {
            if a == b {
                return value_err(format!("line {lineno}: CNOT control equals target"));
            }
        }
        max_q = max_q.max(gate.qubits().into_iter().max().unwrap_or(0));
        gates.push(gate);
    }

    let mut circuit = Circuit::new(max_q + 1);
    for g in gates {
        circuit.push(g)?;
    }
    Ok(circuit)
}

/// One line of CHP output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub qubit: usize,
    pub outcome: bool,
    pub deterministic: bool,
}

/// Runs a circuit on |0…0⟩, returning the measurement outcomes and the
/// final tableau.
pub fn run(circuit: &Circuit, rng: &mut Rng) -> Result<(Vec<Outcome>, Tableau)> {
    let mut tab = Tableau::new(circuit.num_qubits().max(1));
    let results = circuit.run(&mut tab, rng)?;
    let outcomes = results
        .into_iter()
        .map(|(qubit, r)| Outcome {
            qubit,
            outcome: r.outcome,
            deterministic: r.deterministic,
        })
        .collect();
    Ok((outcomes, tab))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TELEPORT: &str = "Teleportation (AG2004 Section IV)\n#\nh 1\nc 1 2\nc 0 1\nh 0\nm 0\nm 1\nc 0 3\nc 1 4\nc 4 2\nh 2\nc 3 2\nh 2\n";

    #[test]
    fn parses_paper_program() {
        let c = parse(TELEPORT).unwrap();
        assert_eq!(c.num_qubits(), 5);
        assert_eq!(c.len(), 12);
        assert_eq!(c.gates()[0], Gate::H(1));
    }

    #[test]
    fn errors_name_the_line() {
        let err = parse("#\nh 0\nq 1\n").unwrap_err();
        assert!(err.to_string().contains("line 3"), "{err}");
        assert!(parse("c 1 1").is_err());
        assert!(parse("h").is_err());
    }

    #[test]
    fn teleports_one() {
        // Prepare |1⟩ on qubit 0 (X = HPPH), teleport, measure qubit 2.
        let prog = format!(
            "#\nh 0\np 0\np 0\nh 0\n{}m 2\n",
            &TELEPORT[TELEPORT.find("#\n").unwrap() + 2..]
        );
        for seed in 0..10 {
            let (outs, _) = run(&parse(&prog).unwrap(), &mut Rng::seed_from_u64(seed)).unwrap();
            let last = outs.last().unwrap();
            assert_eq!(
                (last.qubit, last.outcome, last.deterministic),
                (2, true, true)
            );
        }
    }
}
