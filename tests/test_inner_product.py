"""Section III: inner products of stabilizer states vs. explicit statevectors."""

import numpy as np
import pytest

from reference import apply_gate, zero_state
from stabilizer import (
    Circuit,
    Tableau,
    inner_product,
    inner_product_exponent,
    random_circuit,
)


def unitary_part(n, seed, beta):
    full = random_circuit(n, beta, seed=seed)
    c = Circuit(n)
    gates = [g for g in full.gates() if g[0] != "m"]
    for g in gates:
        getattr(c, {"c": "cnot", "h": "h", "p": "p"}[g[0]])(*g[1:])
    return c, gates


def statevector(n, gates):
    psi = zero_state(n)
    for g in gates:
        psi = apply_gate(psi, g)
    return psi


def test_paper_examples():
    a = Tableau.from_stabilizers(["+XX", "+ZZ"])
    b = Tableau.from_stabilizers(["+ZI", "+IZ"])
    assert inner_product(a, b) == pytest.approx(2**-0.5)
    assert inner_product_exponent(a, b) == 1
    c = Tableau.from_stabilizers(["+ZI", "+ZZ"])  # <ZI, IZ> = <ZI, ZZ>
    assert inner_product(b, c) == pytest.approx(1.0)


def test_orthogonal():
    a = Tableau.from_stabilizers(["+ZI", "+IZ"])
    b = Tableau.from_stabilizers(["+ZI", "-IZ"])
    assert inner_product(a, b) == 0.0
    assert inner_product_exponent(a, b) is None


@pytest.mark.parametrize("n", [1, 2, 3, 4, 5])
def test_random_pairs_match_statevector(n):
    for seed in range(15):
        beta = 0.5 + (seed % 4) * 0.5
        c1, g1 = unitary_part(n, seed, beta)
        c2, g2 = unitary_part(n, 1000 + seed, beta)
        want = abs(np.vdot(statevector(n, g1), statevector(n, g2)))
        got = inner_product(c1.tableau(), c2.tableau())
        assert got == pytest.approx(want, abs=1e-9)
        # Pairs that differ by a few gates hit the 2^(-s/2) cases often.
        c3 = Circuit(n)
        c3.extend(c1)
        c3.h(seed % n)
        c3.p((seed + 1) % n)
        want = abs(np.vdot(statevector(n, g1), statevector(n, g1 + [("h", seed % n), ("p", (seed + 1) % n)])))
        assert inner_product(c1.tableau(), c3.tableau()) == pytest.approx(want, abs=1e-9)
