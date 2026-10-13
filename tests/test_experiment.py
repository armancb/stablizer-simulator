"""Section IV experiment: random beta * n log2 n circuits and measurement timing."""

import math

from stabilizer import random_circuit, time_measurements


def test_random_circuit_shape():
    n, beta = 100, 0.8
    c = random_circuit(n, beta, seed=1)
    gates = c.gates()
    unitary = [g for g in gates if g[0] != "m"]
    assert len(unitary) == math.floor(beta * n * math.log2(n))
    assert [g[1] for g in gates if g[0] == "m"] == list(range(n))
    assert all(g[1] != g[2] for g in gates if g[0] == "c")
    counts = {k: sum(g[0] == k for g in unitary) for k in "chp"}
    assert all(abs(v - len(unitary) / 3) < 0.15 * len(unitary) for v in counts.values())
    assert random_circuit(n, beta, seed=1) == c


def test_dense_tableaus_take_longer_to_measure():
    # The "phase transition" of Fig. 2: sparse (small beta) states are cheap
    # to measure, dense ones cost up to n^2.
    n = 600
    sparse = min(time_measurements(n, 0.2, seed=s) for s in range(3))
    dense = min(time_measurements(n, 2.0, seed=s) for s in range(3))
    assert dense > sparse
