"""Section VI: Lemmas 6 and 7, Theorem 8, and Corollary 9 (PMH CNOT synthesis)."""

import random

import numpy as np
import pytest

from stabilizer import (
    Circuit,
    Tableau,
    canonical_form,
    canonical_rounds,
    cnot_synthesis,
    lemma6_hadamards,
    lemma7,
    random_circuit,
    synthesize,
    to_identity_circuit,
)

SEQUENCE = list("HCPCPCHPCPC")


def random_unitary_circuit(n, seed, beta=3.0):
    full = random_circuit(n, beta, seed=seed)
    c = Circuit(n)
    for gate in full.gates():
        if gate[0] == "c":
            c.cnot(gate[1], gate[2])
        elif gate[0] == "h":
            c.h(gate[1])
        elif gate[0] == "p":
            c.p(gate[1])
    return c


def gf2_rank(m):
    m = np.array(m, dtype=np.uint8) % 2
    rank = 0
    for col in range(m.shape[1]):
        rows = [r for r in range(rank, m.shape[0]) if m[r, col]]
        if not rows:
            continue
        m[[rank, rows[0]]] = m[[rows[0], rank]]
        for r in range(m.shape[0]):
            if r != rank and m[r, col]:
                m[r] ^= m[rank]
        rank += 1
    return rank


@pytest.mark.parametrize("n", [1, 2, 3, 5, 8, 12])
@pytest.mark.parametrize("method", ["gauss", "pmh"])
def test_canonical_form_is_equivalent(n, method):
    for seed in range(5):
        c = random_unitary_circuit(n, seed)
        cf = canonical_form(c, method=method)
        assert cf.tableau() == c.tableau()  # Lemma 5
        rounds = canonical_rounds(c, method=method)
        assert [kind for kind, _ in rounds] == SEQUENCE
        allowed = {"H": "h", "C": "c", "P": "p"}
        for kind, part in rounds:
            assert all(g[0] == allowed[kind] for g in part.gates())


def test_canonical_form_rejects_measurements():
    c = Circuit(1)
    c.measure(0)
    with pytest.raises(ValueError):
        canonical_form(c)


@pytest.mark.parametrize("n", [1, 3, 6, 10])
def test_synthesize_and_to_identity(n):
    for seed in range(5):
        t = random_unitary_circuit(n, 100 + seed).tableau()
        assert synthesize(t).tableau() == t
        g = to_identity_circuit(t)
        u = t.copy()
        u.apply_circuit(g)
        assert u == Tableau(n)


def test_lemma6_makes_x_block_full_rank():
    for seed in range(20):
        n = 1 + seed % 7
        t = random_unitary_circuit(n, seed, beta=1.0).tableau()
        for q in lemma6_hadamards(t):
            t.apply_h(q)
        x, _, _ = t.to_numpy()
        assert gf2_rank(x[n:]) == n


def test_lemma7():
    rng = random.Random(1)
    for n in range(1, 10):
        a = np.zeros((n, n), dtype=np.uint8)
        for i in range(n):
            for j in range(i + 1):
                a[i, j] = a[j, i] = rng.randrange(2)
        lam, m = lemma7(a.tolist())
        m = np.array(m)
        assert np.all(np.triu(m, 1) == 0) and np.all(np.diag(m) == 1)
        assert np.array_equal((a + np.diag(lam)) % 2, (m @ m.T) % 2)
    with pytest.raises(ValueError):
        lemma7([[0, 1], [0, 0]])


def apply_cnots(gates, v):
    v = list(v)
    for c, t in gates:
        v[t] ^= v[c]
    return v


@pytest.mark.parametrize("method", ["gauss", "pmh"])
def test_cnot_synthesis_computes_linear_map(method):
    rng = np.random.default_rng(3)
    for n in [1, 2, 5, 16, 33]:
        while True:
            a = rng.integers(0, 2, size=(n, n))
            if gf2_rank(a) == n:
                break
        gates = cnot_synthesis(a.tolist(), method=method)
        for k in range(n):
            e = [0] * n
            e[k] = 1
            assert apply_cnots(gates, e) == list(a[:, k])


def test_pmh_beats_gauss_on_large_matrices():
    rng = np.random.default_rng(4)
    n = 256
    while True:
        a = rng.integers(0, 2, size=(n, n))
        if gf2_rank(a) == n:
            break
    gauss = len(cnot_synthesis(a.tolist(), method="gauss"))
    pmh = len(cnot_synthesis(a.tolist(), method="pmh"))
    assert pmh < gauss


def test_cnot_synthesis_rejects_singular():
    with pytest.raises(ValueError):
        cnot_synthesis([[1, 1], [1, 1]])
