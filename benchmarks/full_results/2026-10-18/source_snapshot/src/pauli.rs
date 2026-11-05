//! Pauli operators (paper Section II) and the phase function `g` (paper Section III).
//!
//! A single-qubit Pauli is stored as two bits (x, z): I = (0,0), X = (1,0),
//! Z = (0,1), Y = (1,1). Multiplying two Paulis XORs their bits; the only
//! difficulty is the phase, which `g` supplies one qubit at a time.
//!
//! An n-qubit Pauli is packed 64 qubits per `u64` word, LSB-first: qubit `q`
//! lives in word `q / 64`, bit `q % 64`. Padding bits are always zero.

use crate::circuit::Gate;
use crate::error::{index_err, value_err, Result};
use std::fmt;

pub const WORD_BITS: usize = 64;

/// Number of 64-bit words needed for `n` qubits.
#[inline]
pub fn num_words(n: usize) -> usize {
    n.div_ceil(WORD_BITS)
}

/// (word index, bit mask) of qubit `q`.
#[inline]
pub(crate) fn locate(q: usize) -> (usize, u64) {
    (q / WORD_BITS, 1u64 << (q % WORD_BITS))
}

/// The exponent `e` in {-1, 0, 1} with P1 · P2 = i^e · P3, where P1 = (x1, z1),
/// P2 = (x2, z2) and P3 has bits (x1 ⊕ x2, z1 ⊕ z2). P1 is on the left.
///
/// This is exactly the case analysis of Section III.
pub fn g(x1: bool, z1: bool, x2: bool, z2: bool) -> i32 {
    let (x2, z2) = (x2 as i32, z2 as i32);
    match (x1, z1) {
        (false, false) => 0,
        (true, true) => z2 - x2,
        (true, false) => z2 * (2 * x2 - 1),
        (false, true) => x2 * (1 - 2 * z2),
    }
}

/// Σ_j g(x1_j, z1_j, x2_j, z2_j) over whole packed rows.
///
/// `g` is +1 in exactly three cases (Y·Z, X·Y, Z·X) and −1 in three (Y·X, X·Z,
/// Z·Y). Each case is a bit pattern, so one mask per sign marks every qubit
/// in that case and the sum is popcount(plus) − popcount(minus). Every term
/// has a non-negated factor, so padding bits never contribute.
#[inline]
pub fn g_sum(x1: &[u64], z1: &[u64], x2: &[u64], z2: &[u64]) -> i64 {
    let mut plus = 0i64;
    let mut minus = 0i64;
    for k in 0..x1.len() {
        let (a, b, c, d) = (x1[k], z1[k], x2[k], z2[k]);
        let p = (a & b & !c & d) | (a & !b & c & d) | (!a & b & c & !d);
        let m = (a & b & c & !d) | (a & !b & !c & d) | (!a & b & c & d);
        plus += p.count_ones() as i64;
        minus += m.count_ones() as i64;
    }
    plus - minus
}

/// Parity of the symplectic inner product of two packed Paulis:
/// true iff they anticommute.
#[inline]
pub fn anticommute_words(x1: &[u64], z1: &[u64], x2: &[u64], z2: &[u64]) -> bool {
    let mut acc = 0u64;
    for k in 0..x1.len() {
        acc ^= (x1[k] & z2[k]) ^ (z1[k] & x2[k]);
    }
    acc.count_ones() % 2 == 1
}

/// An n-qubit Pauli operator i^phase · P_0 ⊗ … ⊗ P_{n−1}.
///
/// The phase is an exponent of i modulo 4, so 0 is +, 1 is +i, 2 is −,
/// 3 is −i. Hermitian Paulis (all stabilizers) have phase 0 or 2.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PauliString {
    n: usize,
    pub(crate) x: Vec<u64>,
    pub(crate) z: Vec<u64>,
    phase: u8,
}

impl PauliString {
    pub fn identity(n: usize) -> Self {
        let w = num_words(n);
        PauliString {
            n,
            x: vec![0; w],
            z: vec![0; w],
            phase: 0,
        }
    }

    pub(crate) fn from_words(n: usize, x: Vec<u64>, z: Vec<u64>, phase: u8) -> Self {
        debug_assert_eq!(x.len(), num_words(n));
        debug_assert_eq!(z.len(), num_words(n));
        PauliString {
            n,
            x,
            z,
            phase: phase % 4,
        }
    }

    /// A single-qubit Pauli (`'I'`, `'X'`, `'Y'` or `'Z'`) on qubit `q`.
    pub fn single(n: usize, q: usize, letter: char) -> Result<Self> {
        if q >= n {
            return index_err(format!("qubit {q} out of range [0, {n})"));
        }
        let (x, z) = letter_bits(letter)?;
        let mut p = Self::identity(n);
        p.set(q, x, z);
        Ok(p)
    }

    /// Parses strings like `"XIZ"`, `"+XIZ"`, `"-YY"`, `"+iXZ"`, `"-iZ"`.
    /// Qubit 0 is the leftmost letter; `_` is accepted as `I`.
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        let (phase, body) = if let Some(rest) = s.strip_prefix("+i") {
            (1, rest)
        } else if let Some(rest) = s.strip_prefix("-i") {
            (3, rest)
        } else if let Some(rest) = s.strip_prefix('+') {
            (0, rest)
        } else if let Some(rest) = s.strip_prefix('-') {
            (2, rest)
        } else {
            (0, s)
        };
        let letters: Vec<char> = body.chars().collect();
        if letters.is_empty() {
            return value_err(format!("empty Pauli string {s:?}"));
        }
        let mut p = Self::identity(letters.len());
        for (q, &c) in letters.iter().enumerate() {
            let (x, z) = letter_bits(c)?;
            p.set(q, x, z);
        }
        p.phase = phase;
        Ok(p)
    }

    pub fn num_qubits(&self) -> usize {
        self.n
    }

    /// Exponent of i, in 0..4.
    pub fn phase(&self) -> u8 {
        self.phase
    }

    pub fn set_phase(&mut self, phase: u8) {
        self.phase = phase % 4;
    }

    /// Multiplies by i^k.
    pub fn mul_phase(&mut self, k: u8) {
        self.phase = (self.phase + k) % 4;
    }

    pub fn is_hermitian(&self) -> bool {
        self.phase.is_multiple_of(2)
    }

    /// True for phase −1 (and −i).
    pub fn is_negative(&self) -> bool {
        self.phase >= 2
    }

    pub fn x_words(&self) -> &[u64] {
        &self.x
    }

    pub fn z_words(&self) -> &[u64] {
        &self.z
    }

    #[inline]
    pub fn get(&self, q: usize) -> (bool, bool) {
        let (k, m) = locate(q);
        (self.x[k] & m != 0, self.z[k] & m != 0)
    }

    #[inline]
    pub fn set(&mut self, q: usize, x: bool, z: bool) {
        let (k, m) = locate(q);
        if x {
            self.x[k] |= m;
        } else {
            self.x[k] &= !m;
        }
        if z {
            self.z[k] |= m;
        } else {
            self.z[k] &= !m;
        }
    }

    pub fn letter(&self, q: usize) -> char {
        match self.get(q) {
            (false, false) => 'I',
            (true, false) => 'X',
            (true, true) => 'Y',
            (false, true) => 'Z',
        }
    }

    /// The letters without any sign, e.g. `"XIZ"`.
    pub fn letters(&self) -> String {
        (0..self.n).map(|q| self.letter(q)).collect()
    }

    /// True when every letter is I (the phase is ignored).
    pub fn is_identity_letters(&self) -> bool {
        self.x.iter().chain(&self.z).all(|&w| w == 0)
    }

    pub fn weight(&self) -> usize {
        self.x
            .iter()
            .zip(&self.z)
            .map(|(a, b)| (a | b).count_ones() as usize)
            .sum()
    }

    /// Same letters, with phase 0.
    pub fn without_phase(&self) -> Self {
        let mut p = self.clone();
        p.phase = 0;
        p
    }

    pub fn anticommutes(&self, other: &PauliString) -> bool {
        anticommute_words(&self.x, &self.z, &other.x, &other.z)
    }

    pub fn commutes(&self, other: &PauliString) -> bool {
        !self.anticommutes(other)
    }

    /// The product `self · other`, with the phase tracked exactly.
    pub fn mul(&self, other: &PauliString) -> PauliString {
        assert_eq!(
            self.n, other.n,
            "Pauli strings act on different qubit counts"
        );
        let s = g_sum(&self.x, &self.z, &other.x, &other.z);
        let phase = (self.phase as i64 + other.phase as i64 + s).rem_euclid(4) as u8;
        let x = self.x.iter().zip(&other.x).map(|(a, b)| a ^ b).collect();
        let z = self.z.iter().zip(&other.z).map(|(a, b)| a ^ b).collect();
        PauliString {
            n: self.n,
            x,
            z,
            phase,
        }
    }

    /// The same operator with qubit `q` (which must carry `I`) removed.
    pub fn remove_qubit(&self, q: usize) -> PauliString {
        let mut out = PauliString::identity(self.n - 1);
        for j in 0..self.n {
            if j == q {
                continue;
            }
            let (x, z) = self.get(j);
            out.set(if j < q { j } else { j - 1 }, x, z);
        }
        out.phase = self.phase;
        out
    }

    // ------------------------------------------------------------------
    // Conjugation by the stabilizer gates: P ↦ G P G†. These are the
    // tableau update rules of Section III applied to a single row, with the sign
    // flip expressed as adding 2 to the phase.
    // ------------------------------------------------------------------

    pub fn conj_h(&mut self, a: usize) {
        let (x, z) = self.get(a);
        if x && z {
            self.mul_phase(2);
        }
        self.set(a, z, x);
    }

    pub fn conj_p(&mut self, a: usize) {
        let (x, z) = self.get(a);
        if x && z {
            self.mul_phase(2);
        }
        self.set(a, x, z ^ x);
    }

    pub fn conj_cnot(&mut self, a: usize, b: usize) {
        let (xa, za) = self.get(a);
        let (xb, zb) = self.get(b);
        if xa && zb && (xb == za) {
            self.mul_phase(2);
        }
        self.set(b, xb ^ xa, zb);
        self.set(a, xa, za ^ zb);
    }

    /// P ↦ G P G†. Measurement gates are ignored.
    pub fn conj_gate(&mut self, gate: &Gate) {
        match *gate {
            Gate::Cnot(a, b) => self.conj_cnot(a, b),
            Gate::H(a) => self.conj_h(a),
            Gate::P(a) => self.conj_p(a),
            Gate::Measure(_) => {}
        }
    }

    /// P ↦ G† P G (the Heisenberg picture). H and CNOT are self-inverse, and
    /// P† = P³.
    pub fn conj_gate_inverse(&mut self, gate: &Gate) {
        match *gate {
            Gate::P(a) => {
                self.conj_p(a);
                self.conj_p(a);
                self.conj_p(a);
            }
            _ => self.conj_gate(gate),
        }
    }
}

impl fmt::Display for PauliString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = ["+", "+i", "-", "-i"][self.phase as usize];
        write!(f, "{sign}{}", self.letters())
    }
}

fn letter_bits(c: char) -> Result<(bool, bool)> {
    match c.to_ascii_uppercase() {
        'I' | '_' => Ok((false, false)),
        'X' => Ok((true, false)),
        'Y' => Ok((true, true)),
        'Z' => Ok((false, true)),
        _ => value_err(format!("bad Pauli letter {c:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> PauliString {
        PauliString::parse(s).unwrap()
    }

    #[test]
    fn g_matches_multiplication_table() {
        // (P1, P2, g) from XY = iZ, YZ = iX, ZX = iY and their reverses.
        let table = [
            ('X', 'Y', 1),
            ('Y', 'Z', 1),
            ('Z', 'X', 1),
            ('Y', 'X', -1),
            ('Z', 'Y', -1),
            ('X', 'Z', -1),
        ];
        for a in ['I', 'X', 'Y', 'Z'] {
            for b in ['I', 'X', 'Y', 'Z'] {
                let (x1, z1) = letter_bits(a).unwrap();
                let (x2, z2) = letter_bits(b).unwrap();
                let want = table
                    .iter()
                    .find(|(p, q, _)| *p == a && *q == b)
                    .map_or(0, |t| t.2);
                assert_eq!(g(x1, z1, x2, z2), want, "{a}{b}");
            }
        }
    }

    #[test]
    fn parse_display_roundtrip() {
        for s in ["+XYZI", "-IIII", "+iXZ", "-iYY"] {
            assert_eq!(p(s).to_string(), s);
        }
        assert_eq!(p("xz").to_string(), "+XZ");
    }

    #[test]
    fn products() {
        assert_eq!(p("X").mul(&p("Z")).to_string(), "-iY");
        assert_eq!(p("XX").mul(&p("ZZ")).to_string(), "-YY");
        assert_eq!(p("XZ").mul(&p("ZX")).to_string(), "+YY");
        assert_eq!(p("-XZ").mul(&p("ZX")).to_string(), "-YY");
    }

    #[test]
    fn multiword_product_phase() {
        let n = 70;
        let xs = PauliString::parse(&"X".repeat(n)).unwrap();
        let zs = PauliString::parse(&"Z".repeat(n)).unwrap();
        assert_eq!(xs.mul(&zs).to_string(), format!("-{}", "Y".repeat(n)));
    }

    #[test]
    fn conjugation_rules() {
        let mut q = p("X");
        q.conj_h(0);
        assert_eq!(q.to_string(), "+Z");
        let mut q = p("Y");
        q.conj_h(0);
        assert_eq!(q.to_string(), "-Y");
        let mut q = p("X");
        q.conj_p(0);
        assert_eq!(q.to_string(), "+Y");
        let mut q = p("Y");
        q.conj_p(0);
        assert_eq!(q.to_string(), "-X");
        let mut q = p("XI");
        q.conj_cnot(0, 1);
        assert_eq!(q.to_string(), "+XX");
        let mut q = p("IZ");
        q.conj_cnot(0, 1);
        assert_eq!(q.to_string(), "+ZZ");
        // Inverse conjugation undoes forward conjugation.
        for s in ["XY", "YZ", "ZX", "YY"] {
            for gate in [Gate::H(0), Gate::P(1), Gate::Cnot(0, 1), Gate::Cnot(1, 0)] {
                let mut q = p(s);
                q.conj_gate(&gate);
                q.conj_gate_inverse(&gate);
                assert_eq!(q, p(s));
            }
        }
    }

    #[test]
    fn remove_qubit() {
        assert_eq!(p("-XIZ").remove_qubit(1).to_string(), "-XZ");
    }
}
