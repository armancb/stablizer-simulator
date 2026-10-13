"""Section V, Theorem 4: the alpha-tracking procedure agrees with the tableau."""

import pytest

from stabilizer import Circuit, determinate_outcome, random_circuit


def unitary(n, seed, beta):
    c = Circuit(n)
    for g in random_circuit(n, beta, seed=seed).gates():
        if g[0] != "m":
            getattr(c, {"c": "cnot", "h": "h", "p": "p"}[g[0]])(*g[1:])
    return c


@pytest.mark.parametrize("n", [1, 2, 3, 5, 8])
def test_matches_tableau(n):
    determinate = 0
    for seed in range(40):
        c = unitary(n, seed, beta=0.2 + 0.2 * (seed % 6))
        t = c.tableau()
        for a in range(n):
            want = t.peek(a)
            assert determinate_outcome(c, a) == want
            determinate += want is not None
    assert determinate > 0


def test_known_outcomes():
    c = Circuit(2)
    for g in "hpph":  # X on qubit 0
        getattr(c, g)(0)
    c.cnot(0, 1)
    assert determinate_outcome(c, 0) == 1
    assert determinate_outcome(c, 1) == 1
    c.h(0)
    assert determinate_outcome(c, 0) is None


def test_rejects_measurements():
    c = Circuit(1)
    c.measure(0)
    with pytest.raises(ValueError):
        determinate_outcome(c, 0)
