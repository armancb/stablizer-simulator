//! Inner product of two stabilizer states (end of Section III).
//!
//! |⟨ψ|φ⟩| is 0 if the two stabilizer groups contain the same Pauli with
//! opposite signs, and 2^{−s/2} otherwise, where s is the minimum number of
//! generators in which the groups must differ. The algorithm: use Theorem 8
//! to find U with U|ψ⟩ = |0…0⟩, apply U to φ, and Gaussian-eliminate. s is
//! then the rank of the x-block of U|φ⟩'s stabilizers, and the overlap is
//! zero iff some Z-only element has a minus sign. This takes O(n³) time.

use crate::canonical::{to_identity_circuit, CnotSynthesis};
use crate::error::{value_err, Result};
use crate::tableau::Tableau;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InnerProduct {
    /// The states are orthogonal.
    pub zero: bool,
    /// The exponent s in |⟨ψ|φ⟩| = 2^{−s/2}.
    pub s: usize,
}

impl InnerProduct {
    pub fn value(&self) -> f64 {
        if self.zero {
            0.0
        } else {
            (2f64).powf(-(self.s as f64) / 2.0)
        }
    }
}

pub fn inner_product(psi: &Tableau, phi: &Tableau) -> Result<InnerProduct> {
    let n = psi.num_qubits();
    if phi.num_qubits() != n {
        return value_err("states have different numbers of qubits");
    }
    phi.check_invariants()?;
    let u = to_identity_circuit(psi, CnotSynthesis::Gauss)?.to_circuit();
    let mut t = phi.clone();
    for g in u.gates() {
        t.apply_unitary(g)?;
    }

    let mut gens = t.stabilizers();
    let mut row = 0;
    for q in 0..n {
        let Some(piv) = (row..n).find(|&i| gens[i].get(q).0) else {
            continue;
        };
        gens.swap(row, piv);
        for i in 0..n {
            if i != row && gens[i].get(q).0 {
                gens[i] = gens[row].mul(&gens[i]);
            }
        }
        row += 1;
    }
    let zero = gens[row..].iter().any(|g| g.is_negative());
    Ok(InnerProduct { zero, s: row })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pauli::PauliString;

    fn state(gens: &[&str]) -> Tableau {
        let ps: Vec<PauliString> = gens
            .iter()
            .map(|s| PauliString::parse(s).unwrap())
            .collect();
        Tableau::from_stabilizers(&ps).unwrap()
    }

    #[test]
    fn paper_examples() {
        // ⟨XX, ZZ⟩ and ⟨ZI, IZ⟩ have inner product 1/√2.
        let ip = inner_product(&state(&["XX", "ZZ"]), &state(&["ZI", "IZ"])).unwrap();
        assert_eq!(ip, InnerProduct { zero: false, s: 1 });
        assert!((ip.value() - 0.5f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn orthogonal_and_equal() {
        let ip = inner_product(&state(&["ZI", "IZ"]), &state(&["-ZI", "IZ"])).unwrap();
        assert!(ip.zero);
        let s = state(&["XXX", "ZZI", "IZZ"]);
        assert_eq!(inner_product(&s, &s).unwrap().value(), 1.0);
        let plus = state(&["XI", "IX"]);
        assert_eq!(inner_product(&plus, &state(&["ZI", "IZ"])).unwrap().s, 2);
    }
}
