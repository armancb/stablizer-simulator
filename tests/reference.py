"""Slow, obviously-correct numpy references for the tests.

States are tensors with one axis per qubit (axis q = qubit q), so Pauli
strings (qubit 0 leftmost) and numpy.kron block order (first qubit most
significant) both map onto axes without any bit-order bookkeeping.
"""

import numpy as np

I2 = np.eye(2, dtype=complex)
X = np.array([[0, 1], [1, 0]], dtype=complex)
Y = np.array([[0, -1j], [1j, 0]], dtype=complex)
Z = np.array([[1, 0], [0, -1]], dtype=complex)
H = np.array([[1, 1], [1, -1]], dtype=complex) / np.sqrt(2)
S = np.diag([1, 1j]).astype(complex)
T = np.diag([1, np.exp(1j * np.pi / 4)]).astype(complex)
CNOT = np.array(
    [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 1], [0, 0, 1, 0]], dtype=complex
)
PAULI = {"I": I2, "X": X, "Y": Y, "Z": Z}


def parse_pauli(s):
    """'+iXZ' -> (phase, 'XZ')."""
    for prefix, phase in (("+i", 1j), ("-i", -1j), ("+", 1), ("-", -1)):
        if s.startswith(prefix):
            return phase, s[len(prefix):]
    return 1, s


def zero_state(n):
    psi = np.zeros((2,) * n, dtype=complex)
    psi[(0,) * n] = 1
    return psi


def apply(psi, u, qubits):
    """Applies a 2^k x 2^k unitary to the given qubit axes of a state."""
    k = len(qubits)
    u = u.reshape((2,) * (2 * k))
    psi = np.tensordot(u, psi, axes=(list(range(k, 2 * k)), list(qubits)))
    return np.moveaxis(psi, list(range(k)), list(qubits))


def apply_gate(psi, gate):
    """gate is a Circuit.gates() tuple: ('c', a, b), ('h', a), ('p', a)."""
    kind = gate[0]
    if kind == "c":
        return apply(psi, CNOT, [gate[1], gate[2]])
    return apply(psi, {"h": H, "p": S}[kind], [gate[1]])


def apply_pauli(psi, pauli):
    phase, letters = parse_pauli(pauli)
    out = psi
    for q, letter in enumerate(letters):
        if letter != "I":
            out = apply(out, PAULI[letter], [q])
    return phase * out


def prob_zero(psi, q):
    return float(np.sum(np.abs(np.take(psi, 0, axis=q)) ** 2))


def project(psi, q, outcome):
    psi = psi.copy()
    idx = [slice(None)] * psi.ndim
    idx[q] = 1 - outcome
    psi[tuple(idx)] = 0
    norm = np.linalg.norm(psi)
    assert norm > 1e-9, "projected onto a zero-probability outcome"
    return psi / norm


def pauli_matrix(pauli):
    phase, letters = parse_pauli(pauli)
    m = np.array([[1]], dtype=complex)
    for letter in letters:
        m = np.kron(m, PAULI[letter])
    return phase * m


def stabilizer_density(n, stabilizers):
    """rho = 2^-n prod (I + M_i) as a 2^n x 2^n matrix."""
    rho = np.eye(2**n, dtype=complex)
    for s in stabilizers:
        rho = rho @ (np.eye(2**n) + pauli_matrix(s))
    return rho / 2**n


# ---------------------------------------------------------------------- #
# Density matrices as tensors with 2n axes (ket axes 0..n, bra axes n..2n) #
# ---------------------------------------------------------------------- #


def dm_from_matrix(rho, n):
    return rho.reshape((2,) * (2 * n))


def dm_to_matrix(rho, n):
    return rho.reshape(2**n, 2**n)


def dm_apply(rho, u, qubits, n):
    rho = apply(rho, u, qubits)
    return apply(rho, u.conj(), [n + q for q in qubits])


def dm_prob_zero(rho, q, n):
    m = dm_to_matrix(rho, n)
    diag = np.real(np.diag(m)).reshape((2,) * n)
    return float(np.sum(np.take(diag, 0, axis=q)))


def dm_project(rho, q, outcome, n):
    rho = rho.copy()
    for axis in (q, n + q):
        idx = [slice(None)] * rho.ndim
        idx[axis] = 1 - outcome
        rho[tuple(idx)] = 0
    tr = np.real(np.trace(dm_to_matrix(rho, n)))
    assert tr > 1e-9
    return rho / tr


def dm_partial_trace(rho, q, n):
    """Traces out qubit q of an n-qubit density tensor."""
    return np.trace(rho, axis1=q, axis2=n + q)


def random_density(k, rng):
    """A random full-rank 2^k x 2^k density matrix."""
    a = rng.normal(size=(2**k, 2**k)) + 1j * rng.normal(size=(2**k, 2**k))
    rho = a @ a.conj().T
    return rho / np.trace(rho)


def random_unitary(k, rng):
    a = rng.normal(size=(2**k, 2**k)) + 1j * rng.normal(size=(2**k, 2**k))
    q, r = np.linalg.qr(a)
    return q * (np.diag(r) / np.abs(np.diag(r)))
