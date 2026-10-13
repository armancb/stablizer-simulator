"""Section VII A: mixed-state tableau vs. explicit density matrices."""

import random

import numpy as np
import pytest

from reference import (
    CNOT,
    H,
    S,
    dm_apply,
    dm_from_matrix,
    dm_partial_trace,
    dm_prob_zero,
    dm_project,
    dm_to_matrix,
    stabilizer_density,
)
from stabilizer import MixedTableau, Tableau


def initial_density(n, r):
    """|0> on qubits 0..r, I/2 on the rest."""
    zero = np.array([[1, 0], [0, 0]], dtype=complex)
    mixed = np.eye(2, dtype=complex) / 2
    rho = np.array([[1]], dtype=complex)
    for q in range(n):
        rho = np.kron(rho, zero if q < r else mixed)
    return rho


def check(t, rho, n):
    t.check_invariants()
    assert len(t.stabilizers()) == t.rank
    assert np.allclose(stabilizer_density(n, t.stabilizers()), dm_to_matrix(rho, n))


@pytest.mark.parametrize("n", [1, 2, 3, 4])
def test_random_operations_match_density_matrix(n):
    rng = random.Random(10 + n)
    for r in range(n + 1):
        for trial in range(8):
            t = MixedTableau(n, r, seed=trial)
            rho = dm_from_matrix(initial_density(n, r), n)
            check(t, rho, n)
            for _ in range(25):
                a = rng.randrange(n)
                kind = rng.choice("chpmm" if n > 1 else "hpmm")
                if kind == "c":
                    b = rng.choice([q for q in range(n) if q != a])
                    t.apply_cnot(a, b)
                    rho = dm_apply(rho, CNOT, [a, b], n)
                elif kind == "h":
                    t.apply_h(a)
                    rho = dm_apply(rho, H, [a], n)
                elif kind == "p":
                    t.apply_s(a)
                    rho = dm_apply(rho, S, [a], n)
                else:
                    p0 = dm_prob_zero(rho, a, n)
                    peek = t.peek(a)
                    if peek is None:
                        assert p0 == pytest.approx(0.5)
                    else:
                        assert p0 == pytest.approx(1.0 - peek)
                    outcome = t.measure(a)
                    rho = dm_project(rho, a, outcome, n)
                check(t, rho, n)


def test_case_three_increases_rank():
    t = MixedTableau(2, 0, seed=0)
    assert t.rank == 0 and str(t) == "(maximally mixed)"
    t.measure(1, force=1)
    assert t.rank == 1 and t.stabilizers() == ["-IZ"]
    assert t.measure(1) == 1  # now determinate
    t.check_invariants()


def test_from_pure_and_from_stabilizers():
    p = Tableau(2)
    p.apply_h(0)
    p.apply_cnot(0, 1)
    m = MixedTableau.from_pure(p)
    assert m.rank == 2 and m.stabilizers() == ["+XX", "+ZZ"]
    m2 = MixedTableau.from_stabilizers(3, ["+ZZI"])
    assert m2.rank == 1
    m2.check_invariants()
    assert len(m2.rows()) == 6


@pytest.mark.parametrize("n", [2, 3, 4])
def test_discard_matches_partial_trace(n):
    rng = random.Random(n)
    for trial in range(15):
        r = rng.randrange(n + 1)
        t = MixedTableau(n, r, seed=trial)
        rho = dm_from_matrix(initial_density(n, r), n)
        for _ in range(15):
            a = rng.randrange(n)
            if rng.random() < 0.5:
                b = rng.choice([q for q in range(n) if q != a])
                t.apply_cnot(a, b)
                rho = dm_apply(rho, CNOT, [a, b], n)
            else:
                t.apply_h(a)
                rho = dm_apply(rho, H, [a], n)
                t.apply_s(b := rng.randrange(n))
                rho = dm_apply(rho, S, [b], n)
        q = rng.randrange(n)
        t.discard(q)
        rho = dm_partial_trace(rho, q, n)
        assert t.n == n - 1
        check(t, rho, n - 1)


def test_discard_last_qubit_fails():
    with pytest.raises(ValueError):
        MixedTableau(1, 1).discard(0)
