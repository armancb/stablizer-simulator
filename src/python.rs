//! Python bindings (module `stabilizer._core`).
//!
//! Paulis cross the boundary as signed strings ("+XIZ"), qubits and rows as
//! ints, matrices as anything `numpy.asarray` accepts. `Error::Index`
//! becomes `IndexError` and `Error::Value` becomes `ValueError`.

use crate::canonical::{self, CnotSynthesis, RoundCircuit};
use crate::chp;
use crate::circuit::{Circuit, Gate};
use crate::counting;
use crate::error::Error;
use crate::experiment;
use crate::gf2::BitMatrix;
use crate::inner_product as ip;
use crate::mixed::MixedTableau;
use crate::nonstab_gates::NonStabilizerSimulator;
use crate::nonstab_init::ProductStateSimulator;
use crate::parity_l;
use crate::pauli::{self, PauliString};
use crate::pmh;
use crate::rng::Rng;
use crate::tableau::Tableau;
use num_complex::Complex64;
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

/// (qubit, outcome, deterministic) for one measurement.
type Outcome = (usize, u8, bool);

fn to_py(e: Error) -> PyErr {
    match e {
        Error::Index(m) => PyIndexError::new_err(m),
        Error::Value(m) => PyValueError::new_err(m),
    }
}

trait IntoPy<T> {
    fn py(self) -> PyResult<T>;
}

impl<T> IntoPy<T> for crate::error::Result<T> {
    fn py(self) -> PyResult<T> {
        self.map_err(to_py)
    }
}

/// Negative indices are out of range (no Python-style wrap-around).
fn index(v: isize, what: &str) -> PyResult<usize> {
    usize::try_from(v).map_err(|_| PyIndexError::new_err(format!("{what} {v} out of range")))
}

fn pauli(s: &str) -> PyResult<PauliString> {
    PauliString::parse(s).py()
}

fn strings(ps: Vec<PauliString>) -> Vec<String> {
    ps.into_iter().map(|p| p.to_string()).collect()
}

fn force_arg(force: Option<i64>) -> PyResult<Option<bool>> {
    match force {
        None => Ok(None),
        Some(0) => Ok(Some(false)),
        Some(1) => Ok(Some(true)),
        Some(v) => Err(PyValueError::new_err(format!(
            "force must be 0, 1 or None, got {v}"
        ))),
    }
}

fn method_arg(method: &str) -> PyResult<CnotSynthesis> {
    match method.to_ascii_lowercase().as_str() {
        "gauss" => Ok(CnotSynthesis::Gauss),
        "pmh" => Ok(CnotSynthesis::Pmh),
        _ => Err(PyValueError::new_err(format!(
            "method must be 'gauss' or 'pmh', got {method:?}"
        ))),
    }
}

fn bit_matrix(rows: Vec<Vec<u8>>) -> PyResult<BitMatrix> {
    BitMatrix::from_rows(&rows).py()
}

/// A square complex matrix via `numpy.asarray(obj, dtype=complex128)`.
fn complex_matrix(obj: &Bound<'_, PyAny>) -> PyResult<Vec<Complex64>> {
    let py = obj.py();
    let np = py.import("numpy")?;
    let kwargs = PyDict::new(py);
    kwargs.set_item("dtype", "complex128")?;
    let arr = np.getattr("asarray")?.call((obj,), Some(&kwargs))?;
    let shape: Vec<usize> = arr.getattr("shape")?.extract()?;
    if shape.len() != 2 || shape[0] != shape[1] {
        return Err(PyValueError::new_err(format!(
            "expected a square matrix, got shape {shape:?}"
        )));
    }
    let re: Vec<f64> = arr
        .getattr("real")?
        .call_method0("ravel")?
        .call_method0("tolist")?
        .extract()?;
    let im: Vec<f64> = arr
        .getattr("imag")?
        .call_method0("ravel")?
        .call_method0("tolist")?
        .extract()?;
    Ok(re
        .into_iter()
        .zip(im)
        .map(|(a, b)| Complex64::new(a, b))
        .collect())
}

fn gate_tuple<'py>(py: Python<'py>, g: &Gate) -> PyResult<Bound<'py, PyTuple>> {
    let letter = g.letter().to_string();
    match *g {
        Gate::Cnot(a, b) => PyTuple::new(
            py,
            [
                letter.into_pyobject(py)?.into_any(),
                a.into_pyobject(py)?.into_any(),
                b.into_pyobject(py)?.into_any(),
            ],
        ),
        Gate::H(a) | Gate::P(a) | Gate::Measure(a) => PyTuple::new(
            py,
            [
                letter.into_pyobject(py)?.into_any(),
                a.into_pyobject(py)?.into_any(),
            ],
        ),
    }
}

// ----------------------------------------------------------------------
// Tableau (ÂSection III)
// ----------------------------------------------------------------------

/// Aaronsonâ€“Gottesman tableau for n qubits, initialized to |0â€¦0âŸ©.
///
/// Rows 0..n are destabilizers and rows n..2n stabilizers. Random
/// measurement outcomes come from an internal generator seeded with `seed`.
#[pyclass(name = "Tableau", module = "stabilizer._core", skip_from_py_object)]
#[derive(Clone)]
struct PyTableau {
    inner: Tableau,
    rng: Rng,
}

#[pymethods]
impl PyTableau {
    #[new]
    #[pyo3(signature = (n, seed=None))]
    fn new(n: isize, seed: Option<u64>) -> PyResult<Self> {
        if n < 1 {
            return Err(PyValueError::new_err(format!("n must be >= 1, got {n}")));
        }
        Ok(PyTableau {
            inner: Tableau::new(n as usize),
            rng: Rng::from_seed(seed),
        })
    }

    /// The tableau whose stabilizer generators are exactly `stabilizers`.
    #[staticmethod]
    #[pyo3(signature = (stabilizers, seed=None))]
    fn from_stabilizers(stabilizers: Vec<String>, seed: Option<u64>) -> PyResult<Self> {
        let ps = stabilizers
            .iter()
            .map(|s| pauli(s))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyTableau {
            inner: Tableau::from_stabilizers(&ps).py()?,
            rng: Rng::from_seed(seed),
        })
    }

    #[getter]
    fn n(&self) -> usize {
        self.inner.num_qubits()
    }

    #[getter]
    fn num_words(&self) -> usize {
        self.inner.num_words()
    }

    #[pyo3(signature = (seed=None))]
    fn reseed(&mut self, seed: Option<u64>) {
        self.rng = Rng::from_seed(seed);
    }

    fn get_x(&self, row: isize, col: isize) -> PyResult<u8> {
        Ok(self
            .inner
            .get_x(index(row, "row")?, index(col, "qubit")?)
            .py()? as u8)
    }

    fn get_z(&self, row: isize, col: isize) -> PyResult<u8> {
        Ok(self
            .inner
            .get_z(index(row, "row")?, index(col, "qubit")?)
            .py()? as u8)
    }

    fn get_r(&self, row: isize) -> PyResult<u8> {
        Ok(self.inner.get_r(index(row, "row")?).py()? as u8)
    }

    fn set_x(&mut self, row: isize, col: isize, val: i64) -> PyResult<()> {
        self.inner
            .set_x(index(row, "row")?, index(col, "qubit")?, val != 0)
            .py()
    }

    fn set_z(&mut self, row: isize, col: isize, val: i64) -> PyResult<()> {
        self.inner
            .set_z(index(row, "row")?, index(col, "qubit")?, val != 0)
            .py()
    }

    fn set_r(&mut self, row: isize, val: i64) -> PyResult<()> {
        self.inner.set_r(index(row, "row")?, val != 0).py()
    }

    /// Row as a signed Pauli string, e.g. '-XIZY' (qubit 0 leftmost).
    fn row_to_pauli(&self, row: isize) -> PyResult<String> {
        Ok(self.inner.row(index(row, "row")?).py()?.to_string())
    }

    /// Overwrites a row from a signed Pauli string like '+XZ' or '-YI'.
    fn set_row_from_pauli(&mut self, row: isize, pauli_str: &str) -> PyResult<()> {
        let p = pauli(pauli_str)?;
        self.inner.set_row(index(row, "row")?, &p).py()
    }

    /// Row h <- row i * row h with the phase tracked. Raises ValueError (and
    /// leaves row h unchanged) if the rows anticommute.
    fn rowsum(&mut self, h: isize, i: isize) -> PyResult<()> {
        self.inner.rowsum(index(h, "row")?, index(i, "row")?).py()
    }

    #[pyo3(signature = (include_destabilizers=false))]
    fn to_stabilizer_string(&self, include_destabilizers: bool) -> String {
        if include_destabilizers {
            self.inner.to_string()
        } else {
            strings(self.inner.stabilizers()).join("\n")
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Tableau(n={})", self.inner.num_qubits())
    }

    fn copy(&self) -> Self {
        self.clone()
    }

    fn __copy__(&self) -> Self {
        self.clone()
    }

    fn __deepcopy__(&self, _memo: &Bound<'_, PyAny>) -> Self {
        self.clone()
    }

    /// Equal tableaus, signs included (Lemma 5: equivalent unitary circuits).
    fn __eq__(&self, other: PyRef<'_, Self>) -> bool {
        self.inner == other.inner
    }

    fn apply_cnot(&mut self, control: usize, target: usize) -> PyResult<()> {
        self.inner.cnot(control, target).py()
    }

    fn apply_h(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.h(qubit).py()
    }

    /// The phase gate P = diag(1, i).
    fn apply_s(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.p(qubit).py()
    }

    /// Measures `qubit` in the standard basis and returns 0 or 1. With
    /// `force`, a random outcome is post-selected to that value; forcing a
    /// determinate outcome to the other value raises ValueError.
    #[pyo3(signature = (qubit, force=None))]
    fn measure(&mut self, qubit: usize, force: Option<i64>) -> PyResult<u8> {
        let r = match force_arg(force)? {
            Some(b) => self.inner.measure_forced(qubit, b),
            None => self.inner.measure(qubit, &mut self.rng),
        };
        Ok(r.py()?.outcome as u8)
    }

    fn is_deterministic(&self, qubit: usize) -> PyResult<bool> {
        self.inner.is_deterministic(qubit).py()
    }

    /// The determinate outcome of measuring `qubit`, or None if random.
    fn peek(&mut self, qubit: usize) -> PyResult<Option<u8>> {
        Ok(self.inner.peek(qubit).py()?.map(|b| b as u8))
    }

    fn stabilizers(&self) -> Vec<String> {
        strings(self.inner.stabilizers())
    }

    fn destabilizers(&self) -> Vec<String> {
        strings(self.inner.destabilizers())
    }

    /// Reduced row-echelon stabilizer generators: equal iff same state.
    fn canonical_stabilizers(&self) -> Vec<String> {
        strings(self.inner.canonical_stabilizers())
    }

    fn same_state(&self, other: PyRef<'_, Self>) -> bool {
        self.inner.same_state(&other.inner)
    }

    /// Raises ValueError if Proposition 3's commutation invariants fail.
    fn check_invariants(&self) -> PyResult<()> {
        self.inner.check_invariants().py()
    }

    /// (x, z, r) as numpy uint8 arrays of shapes (2n, n), (2n, n), (2n,).
    fn to_numpy<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, Bound<'py, PyAny>, Bound<'py, PyAny>)> {
        let t = &self.inner;
        let n = t.num_qubits();
        let rows = t.rows();
        let xs: Vec<Vec<u32>> = (0..2 * n)
            .map(|i| (0..n).map(|q| rows.x(i, q) as u32).collect())
            .collect();
        let zs: Vec<Vec<u32>> = (0..2 * n)
            .map(|i| (0..n).map(|q| rows.z(i, q) as u32).collect())
            .collect();
        let rs: Vec<u32> = (0..2 * n).map(|i| rows.r(i) as u32).collect();
        let np = py.import("numpy")?;
        let kwargs = PyDict::new(py);
        kwargs.set_item("dtype", "uint8")?;
        let array = np.getattr("array")?;
        Ok((
            array.call((xs,), Some(&kwargs))?,
            array.call((zs,), Some(&kwargs))?,
            array.call((rs,), Some(&kwargs))?,
        ))
    }

    /// Runs a circuit; returns [(qubit, outcome, deterministic)] per measurement.
    fn apply_circuit(&mut self, circuit: PyRef<'_, PyCircuit>) -> PyResult<Vec<Outcome>> {
        let out = circuit.inner.run(&mut self.inner, &mut self.rng).py()?;
        Ok(out
            .into_iter()
            .map(|(q, r)| (q, r.outcome as u8, r.deterministic))
            .collect())
    }
}

// ----------------------------------------------------------------------
// Circuits and CHP (ÂSection IV)
// ----------------------------------------------------------------------

/// A stabilizer circuit: CNOT ('c'), Hadamard ('h'), phase ('p') and
/// measurement ('m') gates on n qubits.
#[pyclass(name = "Circuit", module = "stabilizer._core", skip_from_py_object)]
#[derive(Clone)]
struct PyCircuit {
    inner: Circuit,
}

#[pymethods]
impl PyCircuit {
    #[new]
    fn new(n: usize) -> Self {
        PyCircuit {
            inner: Circuit::new(n),
        }
    }

    #[getter]
    fn n(&self) -> usize {
        self.inner.num_qubits()
    }

    fn cnot(&mut self, control: usize, target: usize) -> PyResult<()> {
        self.inner.cnot(control, target).py()
    }

    fn h(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.h(qubit).py()
    }

    fn p(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.p(qubit).py()
    }

    fn measure(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.measure(qubit).py()
    }

    fn extend(&mut self, other: PyRef<'_, Self>) -> PyResult<()> {
        self.inner.extend(&other.inner).py()
    }

    /// Gates as tuples: ('c', a, b), ('h', a), ('p', a), ('m', a).
    fn gates<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyTuple>>> {
        self.inner
            .gates()
            .iter()
            .map(|g| gate_tuple(py, g))
            .collect()
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn is_unitary(&self) -> bool {
        self.inner.is_unitary()
    }

    fn inverse(&self) -> PyResult<Self> {
        Ok(PyCircuit {
            inner: self.inner.inverse().py()?,
        })
    }

    /// Runs of consecutive same-kind gates, as [(letter, count)].
    fn rounds(&self) -> Vec<(char, usize)> {
        self.inner.rounds()
    }

    /// The tableau this unitary circuit produces from |0â€¦0âŸ©.
    #[pyo3(signature = (seed=None))]
    fn tableau(&self, seed: Option<u64>) -> PyResult<PyTableau> {
        Ok(PyTableau {
            inner: self.inner.tableau().py()?,
            rng: Rng::from_seed(seed),
        })
    }

    fn to_chp(&self) -> String {
        self.inner.to_chp()
    }

    fn __str__(&self) -> String {
        self.inner.to_chp()
    }

    fn __repr__(&self) -> String {
        format!(
            "Circuit(n={}, gates={})",
            self.inner.num_qubits(),
            self.inner.len()
        )
    }

    fn __eq__(&self, other: PyRef<'_, Self>) -> bool {
        self.inner == other.inner
    }
}

/// Parses a CHP program (text before a '#' line is ignored).
#[pyfunction]
fn parse_chp(text: &str) -> PyResult<PyCircuit> {
    Ok(PyCircuit {
        inner: chp::parse(text).py()?,
    })
}

/// Runs a CHP program on |0â€¦0âŸ©. Returns ([(qubit, outcome, deterministic)],
/// final tableau).
#[pyfunction]
#[pyo3(signature = (text, seed=None))]
fn run_chp(text: &str, seed: Option<u64>) -> PyResult<(Vec<Outcome>, PyTableau)> {
    let circuit = chp::parse(text).py()?;
    let mut rng = Rng::from_seed(seed);
    let (outs, tab) = chp::run(&circuit, &mut rng).py()?;
    Ok((
        outs.into_iter()
            .map(|o| (o.qubit, o.outcome as u8, o.deterministic))
            .collect(),
        PyTableau { inner: tab, rng },
    ))
}

/// âŒŠÎ²n logâ‚‚ nâŒ‹ random CNOT/H/P gates, then a measurement of every qubit (ÂSection IV).
#[pyfunction]
#[pyo3(signature = (n, beta, seed=None))]
fn random_circuit(n: usize, beta: f64, seed: Option<u64>) -> PyResult<PyCircuit> {
    Ok(PyCircuit {
        inner: experiment::random_circuit(n, beta, &mut Rng::from_seed(seed)).py()?,
    })
}

/// Seconds per measurement for one random circuit (the Fig. 2 experiment).
#[pyfunction]
#[pyo3(signature = (n, beta, seed=None))]
fn time_measurements(n: usize, beta: f64, seed: Option<u64>) -> PyResult<f64> {
    experiment::time_measurements(n, beta, &mut Rng::from_seed(seed)).py()
}

// ----------------------------------------------------------------------
// Paulis, counting, inner products
// ----------------------------------------------------------------------

/// Exponent e in {-1, 0, 1} with P1 * P2 = i^e * P3 (ÂSection III).
#[pyfunction]
fn g(x1: i64, z1: i64, x2: i64, z2: i64) -> i32 {
    pauli::g(x1 != 0, z1 != 0, x2 != 0, z2 != 0)
}

/// The product a * b of two Pauli strings, e.g. ('+X', '+Z') -> '-iY'.
#[pyfunction]
fn pauli_product(a: &str, b: &str) -> PyResult<String> {
    let (a, b) = (pauli(a)?, pauli(b)?);
    if a.num_qubits() != b.num_qubits() {
        return Err(PyValueError::new_err(
            "Pauli strings have different lengths",
        ));
    }
    Ok(a.mul(&b).to_string())
}

#[pyfunction]
fn pauli_commutes(a: &str, b: &str) -> PyResult<bool> {
    let (a, b) = (pauli(a)?, pauli(b)?);
    if a.num_qubits() != b.num_qubits() {
        return Err(PyValueError::new_err(
            "Pauli strings have different lengths",
        ));
    }
    Ok(a.commutes(&b))
}

/// Proposition 2, as a decimal string (wrapped into an int by the package).
#[pyfunction]
fn _num_stabilizer_states(n: u32) -> String {
    counting::num_stabilizer_states(n).to_string()
}

#[pyfunction]
fn log2_num_stabilizer_states(n: u32) -> f64 {
    counting::log2_num_stabilizer_states(n)
}

/// |<psi|phi>| for two stabilizer states (end of ÂSection III).
#[pyfunction]
fn inner_product(psi: PyRef<'_, PyTableau>, phi: PyRef<'_, PyTableau>) -> PyResult<f64> {
    Ok(ip::inner_product(&psi.inner, &phi.inner).py()?.value())
}

/// s with |<psi|phi>| = 2^(-s/2), or None if the states are orthogonal.
#[pyfunction]
fn inner_product_exponent(
    psi: PyRef<'_, PyTableau>,
    phi: PyRef<'_, PyTableau>,
) -> PyResult<Option<usize>> {
    let ip = ip::inner_product(&psi.inner, &phi.inner).py()?;
    Ok(if ip.zero { None } else { Some(ip.s) })
}

// ----------------------------------------------------------------------
// Canonical form (ÂSection VI) and CNOT synthesis
// ----------------------------------------------------------------------

fn round_list(rc: &RoundCircuit) -> PyResult<Vec<(char, PyCircuit)>> {
    rc.rounds()
        .iter()
        .map(|(kind, gates)| {
            let mut c = Circuit::new(rc.num_qubits());
            for g in gates {
                c.push(*g).py()?;
            }
            Ok((kind.letter(), PyCircuit { inner: c }))
        })
        .collect()
}

/// Theorem 8: an equivalent circuit in H-C-P-C-P-C-H-P-C-P-C order.
/// `method` is 'gauss' or 'pmh' (Corollary 9) for the CNOT rounds.
#[pyfunction]
#[pyo3(signature = (circuit, method="gauss"))]
fn canonical_form(circuit: PyRef<'_, PyCircuit>, method: &str) -> PyResult<PyCircuit> {
    let rc = canonical::canonical_form(&circuit.inner, method_arg(method)?).py()?;
    Ok(PyCircuit {
        inner: rc.to_circuit(),
    })
}

/// The 11 rounds of canonical_form, as [(letter, Circuit)].
#[pyfunction]
#[pyo3(signature = (circuit, method="gauss"))]
fn canonical_rounds(
    circuit: PyRef<'_, PyCircuit>,
    method: &str,
) -> PyResult<Vec<(char, PyCircuit)>> {
    round_list(&canonical::canonical_form(&circuit.inner, method_arg(method)?).py()?)
}

/// Gates G (canonical order) taking `tableau` to the identity tableau.
#[pyfunction]
#[pyo3(signature = (tableau, method="gauss"))]
fn to_identity_circuit(tableau: PyRef<'_, PyTableau>, method: &str) -> PyResult<PyCircuit> {
    let rc = canonical::to_identity_circuit(&tableau.inner, method_arg(method)?).py()?;
    Ok(PyCircuit {
        inner: rc.to_circuit(),
    })
}

/// A unitary circuit whose tableau is `tableau`.
#[pyfunction]
#[pyo3(signature = (tableau, method="gauss"))]
fn synthesize(tableau: PyRef<'_, PyTableau>, method: &str) -> PyResult<PyCircuit> {
    let rc = canonical::synthesize(&tableau.inner, method_arg(method)?).py()?;
    Ok(PyCircuit {
        inner: rc.to_circuit(),
    })
}

/// Lemma 6: qubits whose Hadamards make the stabilizer X-block full rank.
#[pyfunction]
fn lemma6_hadamards(tableau: PyRef<'_, PyTableau>) -> Vec<usize> {
    canonical::lemma6_hadamards(&tableau.inner)
}

/// Lemma 7: for symmetric A returns (Lambda diagonal, M) with A + Lambda = M M^T.
#[pyfunction]
fn lemma7(matrix: Vec<Vec<u8>>) -> PyResult<(Vec<u32>, Vec<Vec<u32>>)> {
    let (lambda, m) = canonical::lemma7(&bit_matrix(matrix)?).py()?;
    let rows = m
        .to_rows()
        .into_iter()
        .map(|r| r.into_iter().map(u32::from).collect())
        .collect();
    Ok((lambda.into_iter().map(u32::from).collect(), rows))
}

/// CNOTs [(control, target)] whose circuit maps basis state |v> to |A v>.
/// `method` is 'pmh' (Patel-Markov-Hayes) or 'gauss'.
#[pyfunction]
#[pyo3(signature = (matrix, method="pmh", section_size=None))]
fn cnot_synthesis(
    matrix: Vec<Vec<u8>>,
    method: &str,
    section_size: Option<usize>,
) -> PyResult<Vec<(usize, usize)>> {
    let a = bit_matrix(matrix)?;
    let ops = match method_arg(method)? {
        CnotSynthesis::Gauss => pmh::gauss_row_ops(&a),
        CnotSynthesis::Pmh => pmh::pmh_row_ops(
            &a,
            section_size.unwrap_or_else(|| pmh::default_section_size(a.rows())),
        ),
    }
    .py()?;
    // Row ops reduce A to I, so A is their product in reverse.
    Ok(ops.into_iter().rev().collect())
}

// ----------------------------------------------------------------------
// ÂSection V
// ----------------------------------------------------------------------

/// Theorem 4's procedure: the determinate outcome of measuring `qubit`
/// after a unitary circuit, or None if it is random.
#[pyfunction]
fn determinate_outcome(circuit: PyRef<'_, PyCircuit>, qubit: usize) -> PyResult<Option<u8>> {
    Ok(parity_l::determinate_outcome(&circuit.inner, qubit)
        .py()?
        .map(|b| b as u8))
}

// ----------------------------------------------------------------------
// ÂSection VII A
// ----------------------------------------------------------------------

/// Stabilizer mixed state (ÂSection VII A): |0> on qubits 0..rank, maximally mixed
/// on the rest.
#[pyclass(
    name = "MixedTableau",
    module = "stabilizer._core",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyMixedTableau {
    inner: MixedTableau,
    rng: Rng,
}

#[pymethods]
impl PyMixedTableau {
    #[new]
    #[pyo3(signature = (n, rank, seed=None))]
    fn new(n: usize, rank: usize, seed: Option<u64>) -> PyResult<Self> {
        Ok(PyMixedTableau {
            inner: MixedTableau::new(n, rank).py()?,
            rng: Rng::from_seed(seed),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (tableau, seed=None))]
    fn from_pure(tableau: PyRef<'_, PyTableau>, seed: Option<u64>) -> Self {
        PyMixedTableau {
            inner: MixedTableau::from_pure(&tableau.inner),
            rng: Rng::from_seed(seed),
        }
    }

    #[staticmethod]
    #[pyo3(signature = (n, stabilizers, seed=None))]
    fn from_stabilizers(n: usize, stabilizers: Vec<String>, seed: Option<u64>) -> PyResult<Self> {
        let ps = stabilizers
            .iter()
            .map(|s| pauli(s))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyMixedTableau {
            inner: MixedTableau::from_stabilizers(n, &ps).py()?,
            rng: Rng::from_seed(seed),
        })
    }

    #[getter]
    fn n(&self) -> usize {
        self.inner.num_qubits()
    }

    #[getter]
    fn rank(&self) -> usize {
        self.inner.rank()
    }

    fn apply_cnot(&mut self, control: usize, target: usize) -> PyResult<()> {
        self.inner.cnot(control, target).py()
    }

    fn apply_h(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.h(qubit).py()
    }

    fn apply_s(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.p(qubit).py()
    }

    #[pyo3(signature = (qubit, force=None))]
    fn measure(&mut self, qubit: usize, force: Option<i64>) -> PyResult<u8> {
        let r = match force_arg(force)? {
            Some(b) => self.inner.measure_forced(qubit, b),
            None => self.inner.measure(qubit, &mut self.rng),
        };
        Ok(r.py()?.outcome as u8)
    }

    fn peek(&mut self, qubit: usize) -> PyResult<Option<u8>> {
        Ok(self.inner.peek(qubit).py()?.map(|b| b as u8))
    }

    /// Traces out `qubit`; higher qubits shift down by one.
    fn discard(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.discard(qubit).py()
    }

    fn stabilizers(&self) -> Vec<String> {
        strings(self.inner.stabilizers())
    }

    fn canonical_stabilizers(&self) -> Vec<String> {
        strings(self.inner.canonical_stabilizers())
    }

    /// All 2n rows: destabilizers, X-bar, stabilizers, Z-bar.
    fn rows(&self) -> Vec<String> {
        strings(self.inner.all_rows())
    }

    fn check_invariants(&self) -> PyResult<()> {
        self.inner.check_invariants().py()
    }

    fn copy(&self) -> Self {
        self.clone()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "MixedTableau(n={}, rank={})",
            self.inner.num_qubits(),
            self.inner.rank()
        )
    }
}

// ----------------------------------------------------------------------
// ÂSection VII B
// ----------------------------------------------------------------------

/// Stabilizer circuit on a tensor product of arbitrary blocks (ÂSection VII B).
/// `blocks` is a list of (qubits, density_matrix); unlisted qubits start in
/// |0>. The first qubit of a block is the most significant bit of its matrix.
#[pyclass(
    name = "ProductStateSimulator",
    module = "stabilizer._core",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyProductStateSimulator {
    inner: ProductStateSimulator,
    rng: Rng,
}

#[pymethods]
impl PyProductStateSimulator {
    #[new]
    #[pyo3(signature = (n, blocks, seed=None))]
    fn new(
        n: usize,
        blocks: Vec<(Vec<usize>, Bound<'_, PyAny>)>,
        seed: Option<u64>,
    ) -> PyResult<Self> {
        let blocks = blocks
            .into_iter()
            .map(|(q, m)| Ok((q, complex_matrix(&m)?)))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyProductStateSimulator {
            inner: ProductStateSimulator::new(n, blocks).py()?,
            rng: Rng::from_seed(seed),
        })
    }

    #[getter]
    fn n(&self) -> usize {
        self.inner.num_qubits()
    }

    #[getter]
    fn num_measurements(&self) -> usize {
        self.inner.num_measurements()
    }

    fn apply_cnot(&mut self, control: usize, target: usize) -> PyResult<()> {
        self.inner.cnot(control, target).py()
    }

    fn apply_h(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.h(qubit).py()
    }

    fn apply_s(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.p(qubit).py()
    }

    /// Probability that measuring `qubit` now gives 0.
    fn probability(&self, qubit: usize) -> PyResult<f64> {
        self.inner.probability(qubit).py()
    }

    #[pyo3(signature = (qubit, force=None))]
    fn measure(&mut self, qubit: usize, force: Option<i64>) -> PyResult<u8> {
        let outcome = match force_arg(force)? {
            Some(b) => {
                self.inner.measure_forced(qubit, b).py()?;
                b
            }
            None => self.inner.measure(qubit, &mut self.rng).py()?,
        };
        Ok(outcome as u8)
    }
}

// ----------------------------------------------------------------------
// ÂSection VII C
// ----------------------------------------------------------------------

/// Stabilizer circuit with a few non-stabilizer gates (ÂSection VII C), from |0â€¦0>.
#[pyclass(
    name = "NonStabilizerSimulator",
    module = "stabilizer._core",
    skip_from_py_object
)]
#[derive(Clone)]
struct PyNonStabilizerSimulator {
    inner: NonStabilizerSimulator,
    rng: Rng,
}

#[pymethods]
impl PyNonStabilizerSimulator {
    #[new]
    #[pyo3(signature = (n, seed=None))]
    fn new(n: usize, seed: Option<u64>) -> PyResult<Self> {
        Ok(PyNonStabilizerSimulator {
            inner: NonStabilizerSimulator::new(n).py()?,
            rng: Rng::from_seed(seed),
        })
    }

    #[getter]
    fn n(&self) -> usize {
        self.inner.num_qubits()
    }

    #[getter]
    fn num_terms(&self) -> usize {
        self.inner.num_terms()
    }

    fn apply_cnot(&mut self, control: usize, target: usize) -> PyResult<()> {
        self.inner.cnot(control, target).py()
    }

    fn apply_h(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.h(qubit).py()
    }

    fn apply_s(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.p(qubit).py()
    }

    fn apply_t(&mut self, qubit: usize) -> PyResult<()> {
        self.inner.t(qubit).py()
    }

    /// Applies a unitary on `qubits`; the first listed qubit is the most
    /// significant bit of the matrix (numpy.kron order).
    fn apply_unitary(&mut self, matrix: Bound<'_, PyAny>, qubits: Vec<usize>) -> PyResult<()> {
        let m = complex_matrix(&matrix)?;
        self.inner.apply_unitary(&m, &qubits).py()
    }

    fn probability(&self, qubit: usize) -> PyResult<f64> {
        self.inner.probability(qubit).py()
    }

    /// Probability that measuring the Pauli `pauli` gives eigenvalue +1.
    fn probability_pauli(&self, pauli_str: &str) -> PyResult<f64> {
        self.inner.probability_pauli(&pauli(pauli_str)?).py()
    }

    #[pyo3(signature = (qubit, force=None))]
    fn measure(&mut self, qubit: usize, force: Option<i64>) -> PyResult<u8> {
        let outcome = match force_arg(force)? {
            Some(b) => self.inner.measure_forced(qubit, b),
            None => self.inner.measure(qubit, &mut self.rng),
        };
        Ok(outcome.py()? as u8)
    }

    /// Measures a Hermitian Pauli; returns 0 for +1 and 1 for -1.
    #[pyo3(signature = (pauli_str, force=None))]
    fn measure_pauli(&mut self, pauli_str: &str, force: Option<i64>) -> PyResult<u8> {
        let q = pauli(pauli_str)?;
        let outcome = match force_arg(force)? {
            Some(b) => self.inner.measure_pauli_forced(&q, b),
            None => self.inner.measure_pauli(&q, &mut self.rng),
        };
        Ok(outcome.py()? as u8)
    }

    fn stabilizers(&self) -> Vec<String> {
        strings(self.inner.stabilizers())
    }
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTableau>()?;
    m.add_class::<PyCircuit>()?;
    m.add_class::<PyMixedTableau>()?;
    m.add_class::<PyProductStateSimulator>()?;
    m.add_class::<PyNonStabilizerSimulator>()?;
    m.add_function(wrap_pyfunction!(g, m)?)?;
    m.add_function(wrap_pyfunction!(pauli_product, m)?)?;
    m.add_function(wrap_pyfunction!(pauli_commutes, m)?)?;
    m.add_function(wrap_pyfunction!(parse_chp, m)?)?;
    m.add_function(wrap_pyfunction!(run_chp, m)?)?;
    m.add_function(wrap_pyfunction!(random_circuit, m)?)?;
    m.add_function(wrap_pyfunction!(time_measurements, m)?)?;
    m.add_function(wrap_pyfunction!(_num_stabilizer_states, m)?)?;
    m.add_function(wrap_pyfunction!(log2_num_stabilizer_states, m)?)?;
    m.add_function(wrap_pyfunction!(inner_product, m)?)?;
    m.add_function(wrap_pyfunction!(inner_product_exponent, m)?)?;
    m.add_function(wrap_pyfunction!(canonical_form, m)?)?;
    m.add_function(wrap_pyfunction!(canonical_rounds, m)?)?;
    m.add_function(wrap_pyfunction!(to_identity_circuit, m)?)?;
    m.add_function(wrap_pyfunction!(synthesize, m)?)?;
    m.add_function(wrap_pyfunction!(lemma6_hadamards, m)?)?;
    m.add_function(wrap_pyfunction!(lemma7, m)?)?;
    m.add_function(wrap_pyfunction!(cnot_synthesis, m)?)?;
    m.add_function(wrap_pyfunction!(determinate_outcome, m)?)?;
    Ok(())
}
