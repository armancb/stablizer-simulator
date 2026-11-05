"""Aaronson-Gottesman (2004) stabilizer circuit simulation, implemented in Rust.

Paper section -> API:

    III   Tableau, g, pauli_product, pauli_commutes, inner_product(_exponent)
    IV    Circuit, parse_chp, run_chp, random_circuit, time_measurements
    V     determinate_outcome
    VI    canonical_form, canonical_rounds, to_identity_circuit, synthesize,
          lemma6_hadamards, lemma7, cnot_synthesis
    VII A MixedTableau
    VII B ProductStateSimulator
    VII C NonStabilizerSimulator
    II    num_stabilizer_states, log2_num_stabilizer_states
"""

from . import _core
from ._core import (
    Circuit,
    MixedTableau,
    NonStabilizerSimulator,
    ProductStateSimulator,
    Tableau,
    canonical_form,
    canonical_rounds,
    cnot_synthesis,
    determinate_outcome,
    g,
    inner_product,
    inner_product_exponent,
    lemma6_hadamards,
    lemma7,
    log2_num_stabilizer_states,
    parse_chp,
    pauli_commutes,
    pauli_product,
    random_circuit,
    run_chp,
    synthesize,
    time_measurements,
    to_identity_circuit,
)


def num_stabilizer_states(n: int) -> int:
    """Proposition 2: the number of pure n-qubit stabilizer states."""
    return int(_core._num_stabilizer_states(n))


__all__ = [
    "Circuit",
    "MixedTableau",
    "NonStabilizerSimulator",
    "ProductStateSimulator",
    "Tableau",
    "canonical_form",
    "canonical_rounds",
    "cnot_synthesis",
    "determinate_outcome",
    "g",
    "inner_product",
    "inner_product_exponent",
    "lemma6_hadamards",
    "lemma7",
    "log2_num_stabilizer_states",
    "num_stabilizer_states",
    "parse_chp",
    "pauli_commutes",
    "pauli_product",
    "random_circuit",
    "run_chp",
    "synthesize",
    "time_measurements",
    "to_identity_circuit",
]
