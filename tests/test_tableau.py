"""Tableau layout, accessors, debug printer, and full-row rowsum tests."""

import copy
import random

import pytest

from stabilizer import Tableau, g, pauli_commutes, pauli_product


# ---------------------------------------------------------------------- #
# Reference: multiply signed Pauli strings with an explicit complex phase #
# ---------------------------------------------------------------------- #

BITS = {"I": (0, 0), "X": (1, 0), "Z": (0, 1), "Y": (1, 1)}
LETTER = {v: k for k, v in BITS.items()}


def pauli_mul(a: str, b: str) -> tuple[complex, str]:
    """(phase, letters) for a * b, where a and b are signed strings like '-XZ'."""
    phase = complex((-1 if a[0] == "-" else 1) * (-1 if b[0] == "-" else 1))
    out = []
    for pa, pb in zip(a[1:], b[1:]):
        x1, z1 = BITS[pa]
        x2, z2 = BITS[pb]
        phase *= 1j ** g(x1, z1, x2, z2)
        out.append(LETTER[(x1 ^ x2, z1 ^ z2)])
    return phase, "".join(out)


def commute(a: str, b: str) -> bool:
    anti = sum(1 for pa, pb in zip(a[1:], b[1:]) if "I" not in (pa, pb) and pa != pb)
    return anti % 2 == 0


def random_pauli(n: int, rng: random.Random) -> str:
    return rng.choice("+-") + "".join(rng.choice("IXYZ") for _ in range(n))


# ---------------------------------------------------------------------- #
# Layout / init                                                           #
# ---------------------------------------------------------------------- #


@pytest.mark.parametrize("n", [1, 2, 3, 63, 64, 65, 130])
def test_identity_init(n):
    t = Tableau(n)
    for row in range(2 * n):
        assert t.get_r(row) == 0
        for q in range(n):
            assert t.get_x(row, q) == (1 if row == q else 0)
            assert t.get_z(row, q) == (1 if row == n + q else 0)


def test_identity_printer():
    t = Tableau(3)
    assert t.to_stabilizer_string() == "+ZII\n+IZI\n+IIZ"
    assert t.to_stabilizer_string(include_destabilizers=True) == (
        "+XII\n+IXI\n+IIX\n----\n+ZII\n+IZI\n+IIZ"
    )
    assert str(t) == t.to_stabilizer_string(include_destabilizers=True)


@pytest.mark.parametrize("n", [5, 64, 100])
def test_accessors_roundtrip(n):
    t = Tableau(n)
    rng = random.Random(n)
    for _ in range(200):
        row, q, v = rng.randrange(2 * n), rng.randrange(n), rng.randrange(2)
        t.set_x(row, q, v)
        assert t.get_x(row, q) == v
        t.set_z(row, q, v)
        assert t.get_z(row, q) == v
    t.set_r(0, 1)
    assert t.get_r(0) == 1
    t.set_r(0, 0)
    assert t.get_r(0) == 0


def test_accessor_bounds():
    t = Tableau(2)
    with pytest.raises(IndexError):
        t.get_x(4, 0)
    with pytest.raises(IndexError):
        t.set_z(0, 2, 1)
    with pytest.raises(IndexError):
        t.get_z(-1, 0)
    with pytest.raises(ValueError):
        Tableau(0)


def test_set_row_from_pauli_roundtrip():
    t = Tableau(4)
    for p in ["+XYZI", "-IIII", "-YYYY", "+ZXZX"]:
        t.set_row_from_pauli(5, p)
        assert t.row_to_pauli(5) == p
    t.set_row_from_pauli(5, "xyzi")  # missing sign means +, case-insensitive
    assert t.row_to_pauli(5) == "+XYZI"
    with pytest.raises(ValueError):
        t.set_row_from_pauli(5, "+XYZ")
    with pytest.raises(ValueError):
        t.set_row_from_pauli(5, "+XYZQ")


def test_copy_is_independent():
    t = Tableau(3)
    for c in (t.copy(), copy.copy(t), copy.deepcopy(t)):
        c.apply_h(0)
        assert t.row_to_pauli(0) == "+XII"
        assert c.row_to_pauli(0) == "+ZII"
        assert c != t


# ---------------------------------------------------------------------- #
# rowsum on hand-constructed products                                     #
# ---------------------------------------------------------------------- #


@pytest.mark.parametrize(
    "row_i,row_h,expected",
    [
        ("+X", "+X", "+I"),
        ("+Z", "+Z", "+I"),
        ("+Y", "+Y", "+I"),
        ("-X", "+X", "-I"),
        ("-Y", "-Y", "+I"),
        ("+XX", "+ZZ", "-YY"),   # (XZ)(XZ) = (-iY)(-iY) = -YY
        ("+ZZ", "+XX", "-YY"),   # (ZX)(ZX) = (iY)(iY)  = -YY
        ("+XX", "+YY", "-ZZ"),   # (iZ)(iZ)
        ("+YY", "+ZZ", "-XX"),   # (iX)(iX)
        ("+XZ", "+ZX", "+YY"),   # (-iY)(iY) = +YY
        ("-XZ", "+ZX", "-YY"),
        ("+XI", "+IZ", "+XZ"),   # disjoint support
        ("+XYZ", "+XYZ", "+III"),
        ("+XXX", "+ZZI", "-YYX"),
    ],
)
def test_rowsum_hand_products(row_i, row_h, expected):
    """rowsum(h, i) sets row h to row_i * row_h."""
    n = len(row_i) - 1
    t = Tableau(n)
    h, i = n, n + 1 if n > 1 else 0  # any two distinct rows
    t.set_row_from_pauli(i, row_i)
    t.set_row_from_pauli(h, row_h)
    t.rowsum(h, i)
    assert t.row_to_pauli(h) == expected
    assert t.row_to_pauli(i) == row_i  # source row untouched
    assert pauli_product(row_i, row_h) == expected


def test_rowsum_anticommuting_raises_and_leaves_row_untouched():
    # X * Z = -iY: imaginary phase, not representable with one phase bit.
    t = Tableau(2)
    t.set_row_from_pauli(0, "+XI")
    t.set_row_from_pauli(2, "+ZI")
    phase, letters = pauli_mul("+XI", "+ZI")
    assert (phase, letters) == (-1j, "YI")  # reference agrees X*Z = -iY
    assert pauli_product("+XI", "+ZI") == "-iYI"
    with pytest.raises(ValueError):
        t.rowsum(2, 0)
    assert t.row_to_pauli(2) == "+ZI"


def test_rowsum_multiword_phase():
    # 70 qubits spans two words; XX...X * ZZ...Z = (-i)^70 YY...Y = -YY...Y
    n = 70
    t = Tableau(n)
    t.set_row_from_pauli(0, "+" + "X" * n)
    t.set_row_from_pauli(n, "+" + "Z" * n)
    t.rowsum(n, 0)
    assert t.row_to_pauli(n) == "-" + "Y" * n


# ---------------------------------------------------------------------- #
# rowsum vs. reference on random commuting pairs                          #
# ---------------------------------------------------------------------- #


@pytest.mark.parametrize("n", [1, 2, 5, 63, 64, 65, 129])
def test_rowsum_random_commuting_pairs(n):
    rng = random.Random(1000 + n)
    t = Tableau(n)
    checked = 0
    while checked < 200:
        a, b = random_pauli(n, rng), random_pauli(n, rng)
        if not commute(a, b):
            continue
        assert pauli_commutes(a, b)
        t.set_row_from_pauli(0, a)
        t.set_row_from_pauli(1 if n > 1 else n, b)
        h = 1 if n > 1 else n
        t.rowsum(h, 0)
        phase, letters = pauli_mul(a, b)
        assert phase in (1, -1)
        assert t.row_to_pauli(h) == ("+" if phase == 1 else "-") + letters
        checked += 1


@pytest.mark.parametrize("n", [3, 64, 65])
def test_rowsum_random_anticommuting_pairs_raise(n):
    rng = random.Random(2000 + n)
    t = Tableau(n)
    checked = 0
    while checked < 50:
        a, b = random_pauli(n, rng), random_pauli(n, rng)
        if commute(a, b):
            continue
        assert not pauli_commutes(a, b)
        t.set_row_from_pauli(0, a)
        t.set_row_from_pauli(1, b)
        with pytest.raises(ValueError):
            t.rowsum(1, 0)
        assert t.row_to_pauli(1) == b
        checked += 1


def test_to_numpy_matches_accessors():
    t = Tableau(3)
    t.apply_h(0)
    t.apply_cnot(0, 2)
    x, z, r = t.to_numpy()
    assert x.shape == (6, 3) and z.shape == (6, 3) and r.shape == (6,)
    for row in range(6):
        assert r[row] == t.get_r(row)
        for q in range(3):
            assert x[row, q] == t.get_x(row, q)
            assert z[row, q] == t.get_z(row, q)


def test_from_stabilizers():
    t = Tableau.from_stabilizers(["+XXX", "+ZZI", "-IZZ"])
    t.check_invariants()
    assert t.stabilizers() == ["+XXX", "+ZZI", "-IZZ"]
    with pytest.raises(ValueError):
        Tableau.from_stabilizers(["+XI", "+ZI"])  # anticommute
    with pytest.raises(ValueError):
        Tableau.from_stabilizers(["+ZI", "+ZI"])  # dependent
