"""Section III gates and measurement checked against a numpy statevector."""

import random

import numpy as np
import pytest

from reference import apply_gate, apply_pauli, prob_zero, project, zero_state
from stabilizer import Circuit, Tableau


def random_gate(n, rng):
    kind = rng.choice("chp" if n > 1 else "hp")
    a = rng.randrange(n)
    if kind == "c":
        b = rng.choice([q for q in range(n) if q != a])
        return ("c", a, b)
    return (kind, a)


def apply_to_tableau(t, gate):
    if gate[0] == "c":
        t.apply_cnot(gate[1], gate[2])
    elif gate[0] == "h":
        t.apply_h(gate[1])
    else:
        t.apply_s(gate[1])


def assert_stabilizes(t, psi):
    for s in t.stabilizers():
        assert np.allclose(apply_pauli(psi, s), psi), s


@pytest.mark.parametrize("n", [1, 2, 3, 4, 6])
def test_random_circuits_with_measurements(n):
    rng = random.Random(n)
    for trial in range(30):
        t = Tableau(n, seed=trial)
        psi = zero_state(n)
        for _ in range(40):
            if rng.random() < 0.2:
                q = rng.randrange(n)
                p0 = prob_zero(psi, q)
                deterministic = t.is_deterministic(q)
                assert deterministic == (abs(p0 - 0.5) > 0.25)
                if deterministic:
                    assert t.peek(q) == (0 if p0 > 0.5 else 1)
                else:
                    assert p0 == pytest.approx(0.5)
                outcome = t.measure(q)
                psi = project(psi, q, outcome)
            else:
                gate = random_gate(n, rng)
                apply_to_tableau(t, gate)
                psi = apply_gate(psi, gate)
            t.check_invariants()
            assert_stabilizes(t, psi)


def test_ghz_outcomes_agree():
    for seed in range(20):
        t = Tableau(3, seed=seed)
        t.apply_h(0)
        t.apply_cnot(0, 1)
        t.apply_cnot(0, 2)
        first = t.measure(0)
        assert t.is_deterministic(1) and t.is_deterministic(2)
        assert t.measure(1) == first
        assert t.measure(2) == first


def test_outcomes_are_fair_and_seeded():
    ones = 0
    for seed in range(400):
        t = Tableau(1, seed=seed)
        t.apply_h(0)
        ones += t.measure(0)
    assert 150 < ones < 250
    a, b = Tableau(1, seed=5), Tableau(1, seed=5)
    a.apply_h(0)
    b.apply_h(0)
    assert a.measure(0) == b.measure(0)


def test_force():
    t = Tableau(1)
    with pytest.raises(ValueError):
        t.measure(0, force=1)  # |0> is determinately 0
    assert t.measure(0, force=0) == 0
    t.apply_h(0)
    assert t.measure(0, force=1) == 1
    assert t.peek(0) == 1
    with pytest.raises(ValueError):
        t.measure(0, force=2)


def test_gate_bounds():
    t = Tableau(2)
    with pytest.raises(IndexError):
        t.apply_h(2)
    with pytest.raises(ValueError):
        t.apply_cnot(1, 1)


def test_apply_circuit_and_tableau():
    c = Circuit(2)
    c.h(0)
    c.cnot(0, 1)
    c.measure(0)
    c.measure(1)
    t = Tableau(2, seed=3)
    out = t.apply_circuit(c)
    assert [q for q, _, _ in out] == [0, 1]
    assert out[0][1] == out[1][1]
    assert (out[0][2], out[1][2]) == (False, True)
    u = Circuit(2)
    u.h(0)
    u.cnot(0, 1)
    assert u.tableau().stabilizers() == ["+XX", "+ZZ"]
    assert c.gates() == [("h", 0), ("c", 0, 1), ("m", 0), ("m", 1)]


def test_same_state_ignores_destabilizers():
    a = Tableau.from_stabilizers(["+XX", "+ZZ"])
    b = Tableau(2)
    b.apply_h(0)
    b.apply_cnot(0, 1)
    assert a.same_state(b)
    b.apply_s(0)
    assert not a.same_state(b)
