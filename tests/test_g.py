"""All 16 cases of g(x1, z1, x2, z2), hand-derived from the Pauli table.

P1 * P2 = i^g * P3. Encoding (x, z): I=(0,0), X=(1,0), Z=(0,1), Y=(1,1).
"""

import itertools

import numpy as np
import pytest

from stabilizer import g

# (P1, P2, expected g, product written out)
HAND_TABLE = [
    ("I", "I", 0),   # I I = I
    ("I", "X", 0),   # I X = X
    ("I", "Y", 0),   # I Y = Y
    ("I", "Z", 0),   # I Z = Z
    ("X", "I", 0),   # X I = X
    ("X", "X", 0),   # X X = I
    ("X", "Y", 1),   # X Y = iZ
    ("X", "Z", -1),  # X Z = -iY
    ("Y", "I", 0),   # Y I = Y
    ("Y", "X", -1),  # Y X = -iZ
    ("Y", "Y", 0),   # Y Y = I
    ("Y", "Z", 1),   # Y Z = iX
    ("Z", "I", 0),   # Z I = Z
    ("Z", "X", 1),   # Z X = iY
    ("Z", "Y", -1),  # Z Y = -iX
    ("Z", "Z", 0),   # Z Z = I
]

BITS = {"I": (0, 0), "X": (1, 0), "Z": (0, 1), "Y": (1, 1)}

MATS = {
    "I": np.eye(2, dtype=complex),
    "X": np.array([[0, 1], [1, 0]], dtype=complex),
    "Y": np.array([[0, -1j], [1j, 0]], dtype=complex),
    "Z": np.array([[1, 0], [0, -1]], dtype=complex),
}
BY_BITS = {v: k for k, v in BITS.items()}


def test_hand_table_covers_all_16():
    assert len({(a, b) for a, b, _ in HAND_TABLE}) == 16


@pytest.mark.parametrize("p1,p2,expected", HAND_TABLE)
def test_g_matches_hand_table(p1, p2, expected):
    x1, z1 = BITS[p1]
    x2, z2 = BITS[p2]
    assert g(x1, z1, x2, z2) == expected


@pytest.mark.parametrize("x1,z1,x2,z2", list(itertools.product((0, 1), repeat=4)))
def test_g_matches_matrix_multiplication(x1, z1, x2, z2):
    """Independent cross-check of the hand table using actual 2x2 matrices."""
    product = MATS[BY_BITS[(x1, z1)]] @ MATS[BY_BITS[(x2, z2)]]
    p3 = MATS[BY_BITS[(x1 ^ x2, z1 ^ z2)]]
    e = g(x1, z1, x2, z2)
    assert np.allclose(product, (1j ** e) * p3)
