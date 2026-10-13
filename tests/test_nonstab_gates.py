"""Section VII C: circuits with a few non-stabilizer gates vs. a statevector."""

import random

import numpy as np
import pytest

from reference import (
    CNOT,
    H,
    S,
    T,
    apply,
    apply_pauli,
    prob_zero,
    project,
    random_unitary,
    zero_state,
)
from stabilizer import NonStabilizerSimulator


def pauli_prob_plus(psi, pauli):
    """Probability of eigenvalue +1 when measuring a Hermitian Pauli."""
    return float(np.real(1 + np.vdot(psi, apply_pauli(psi, pauli)))) / 2


def project_pauli(psi, pauli, outcome):
    sign = -1 if outcome else 1
    out = (psi + sign * apply_pauli(psi, pauli)) / 2
    return out / np.linalg.norm(out)


@pytest.mark.parametrize("n", [1, 2, 3, 4])
def test_random_circuits_match_statevector(n):
    rng = random.Random(n)
    nprng = np.random.default_rng(n)
    for trial in range(10):
        sim = NonStabilizerSimulator(n, seed=trial)
        psi = zero_state(n)
        nonstab = 0
        for _ in range(20):
            a = rng.randrange(n)
            kind = rng.choice("chpptuzm" if n > 1 else "hpptuzm")
            if kind == "c":
                b = rng.choice([q for q in range(n) if q != a])
                sim.apply_cnot(a, b)
                psi = apply(psi, CNOT, [a, b])
            elif kind == "h":
                sim.apply_h(a)
                psi = apply(psi, H, [a])
            elif kind == "p":
                sim.apply_s(a)
                psi = apply(psi, S, [a])
            elif kind in "tu" and nonstab < 3:
                nonstab += 1
                if kind == "t":
                    sim.apply_t(a)
                    psi = apply(psi, T, [a])
                else:
                    qs = [a] if n == 1 or rng.random() < 0.5 else [a, rng.choice([q for q in range(n) if q != a])]
                    u = random_unitary(len(qs), nprng)
                    sim.apply_unitary(u, qs)
                    psi = apply(psi, u, qs)
            elif kind == "z":
                pauli = rng.choice("+-") + "".join(rng.choice("IXYZ") for _ in range(n))
                if set(pauli[1:]) == {"I"}:
                    continue
                want = pauli_prob_plus(psi, pauli)
                assert sim.probability_pauli(pauli) == pytest.approx(want, abs=1e-9)
                outcome = sim.measure_pauli(pauli)
                psi = project_pauli(psi, pauli, outcome)
            elif kind == "m":
                assert sim.probability(a) == pytest.approx(prob_zero(psi, a), abs=1e-9)
                outcome = sim.measure(a)
                psi = project(psi, a, outcome)
            for q in range(n):
                assert sim.probability(q) == pytest.approx(prob_zero(psi, q), abs=1e-9)


def test_hth():
    sim = NonStabilizerSimulator(1)
    sim.apply_h(0)
    sim.apply_t(0)
    sim.apply_h(0)
    assert sim.probability(0) == pytest.approx((1 + 2**-0.5) / 2)


def test_term_count_bounded():
    # Each 1-qubit gate multiplies the term count by at most 4^2 = 16.
    sim = NonStabilizerSimulator(3)
    for q in range(3):
        sim.apply_h(q)
        sim.apply_t(q)
    assert 1 <= sim.num_terms <= 16**3


def test_forced_zero_probability_raises():
    sim = NonStabilizerSimulator(1)
    with pytest.raises(ValueError):
        sim.measure(0, force=1)
    with pytest.raises(ValueError):
        sim.apply_unitary(np.ones((2, 2)), [0])  # not unitary
    with pytest.raises(ValueError):
        sim.measure_pauli("+iZ")  # not Hermitian
