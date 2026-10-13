"""Section VII B: stabilizer circuits on tensor-product non-stabilizer inputs."""

import random

import numpy as np
import pytest

from reference import (
    CNOT,
    H,
    S,
    dm_apply,
    dm_from_matrix,
    dm_prob_zero,
    dm_project,
    random_density,
)
from stabilizer import ProductStateSimulator


def random_blocks(n, rng, nprng):
    """Partition 0..n into blocks of 1 or 2 qubits with random states."""
    qubits = list(range(n))
    rng.shuffle(qubits)
    blocks = []
    while qubits:
        k = min(len(qubits), rng.choice([1, 2]))
        part, qubits = qubits[:k], qubits[k:]
        blocks.append((part, random_density(k, nprng)))
    return blocks


def full_density(n, blocks):
    """Assemble the tensor product, honoring each block's qubit order."""
    rho = np.array([[1]], dtype=complex)
    order = []
    for part, m in blocks:
        rho = np.kron(rho, m)
        order.extend(part)
    t = rho.reshape((2,) * (2 * n))
    perm = [order.index(q) for q in range(n)]
    return np.transpose(t, perm + [n + p for p in perm])


@pytest.mark.parametrize("n", [1, 2, 3, 4])
def test_probabilities_match_density_matrix(n):
    rng = random.Random(n)
    nprng = np.random.default_rng(n)
    for trial in range(10):
        blocks = random_blocks(n, rng, nprng)
        sim = ProductStateSimulator(n, blocks, seed=trial)
        rho = full_density(n, blocks)
        for _ in range(12):
            a = rng.randrange(n)
            kind = rng.choice("chpm" if n > 1 else "hpm")
            if kind == "c":
                b = rng.choice([q for q in range(n) if q != a])
                sim.apply_cnot(a, b)
                rho = dm_apply(rho, CNOT, [a, b], n)
            elif kind == "h":
                sim.apply_h(a)
                rho = dm_apply(rho, H, [a], n)
            elif kind == "p":
                sim.apply_s(a)
                rho = dm_apply(rho, S, [a], n)
            elif sim.num_measurements < 4:
                assert sim.probability(a) == pytest.approx(dm_prob_zero(rho, a, n), abs=1e-9)
                outcome = sim.measure(a)
                rho = dm_project(rho, a, outcome, n)
            for q in range(n):
                assert sim.probability(q) == pytest.approx(dm_prob_zero(rho, q, n), abs=1e-9)


def test_default_blocks_are_zero():
    sim = ProductStateSimulator(2, [])
    assert sim.probability(0) == 1.0
    sim.apply_h(1)
    assert sim.probability(1) == pytest.approx(0.5)


def test_magic_state_injection_statistics():
    # T|+> magic state on qubit 0: measuring X gives 0 with prob (1 + 1/sqrt2)/2.
    psi = np.array([1, np.exp(1j * np.pi / 4)]) / np.sqrt(2)
    rho = np.outer(psi, psi.conj())
    sim = ProductStateSimulator(1, [([0], rho)])
    sim.apply_h(0)
    assert sim.probability(0) == pytest.approx((1 + 2**-0.5) / 2)


def test_validation():
    with pytest.raises(ValueError):
        ProductStateSimulator(1, [([0], np.eye(2))])  # trace 2
    with pytest.raises(ValueError):
        ProductStateSimulator(2, [([0], np.eye(2) / 2), ([0], np.eye(2) / 2)])
    with pytest.raises(IndexError):
        ProductStateSimulator(1, [([3], np.eye(2) / 2)])
    sim = ProductStateSimulator(1, [])
    with pytest.raises(ValueError):
        sim.measure(0, force=1)
