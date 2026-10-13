"""Proposition 2, checked by enumerating every reachable stabilizer state."""

import math

import pytest

from stabilizer import Tableau, log2_num_stabilizer_states, num_stabilizer_states


def test_formula_values():
    assert [num_stabilizer_states(n) for n in range(1, 5)] == [6, 60, 1080, 36720]
    n = 40
    want = 2**n * math.prod(2 ** (n - k) + 1 for k in range(n))
    assert num_stabilizer_states(n) == want
    assert log2_num_stabilizer_states(n) == pytest.approx(math.log2(want))


@pytest.mark.parametrize("n", [1, 2, 3])
def test_enumeration_matches_formula(n):
    """Breadth-first search over H, P and CNOT from |0...0> (Theorem 1)."""
    start = Tableau(n)
    seen = {tuple(start.canonical_stabilizers())}
    frontier = [start]
    while frontier:
        nxt = []
        for t in frontier:
            moves = [("h", q) for q in range(n)] + [("s", q) for q in range(n)]
            moves += [("c", a, b) for a in range(n) for b in range(n) if a != b]
            for m in moves:
                u = t.copy()
                if m[0] == "h":
                    u.apply_h(m[1])
                elif m[0] == "s":
                    u.apply_s(m[1])
                else:
                    u.apply_cnot(m[1], m[2])
                key = tuple(u.canonical_stabilizers())
                if key not in seen:
                    seen.add(key)
                    nxt.append(u)
        frontier = nxt
    assert len(seen) == num_stabilizer_states(n)
