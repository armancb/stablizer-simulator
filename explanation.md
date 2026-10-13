# Explanation: simulating stabilizer circuits (AG2004) in Rust and Python

This document explains the project from the ground up. It does **not** assume you have written Rust, know linear algebra over bits, or have read the paper. Each idea is introduced before it is used, and most sections end with a small worked example you can check by hand.

Three kinds of boxes appear throughout:

- **Derivation** boxes work the maths out step by step, so no formula has to be taken on trust.
- **Reading the code** boxes show the real Rust source for an idea and explain its syntax line by line. The snippets are copied from `src/`, with comments added and occasionally a line re-wrapped to fit.
- Section 0 is a crash course in Rust syntax. Read it first if you have never seen Rust; the code boxes refer back to it.

The paper is Scott Aaronson and Daniel Gottesman, *Improved Simulation of Stabilizer Circuits* (2004), saved as `AG2004.pdf`. The paper's sections are referred to as "Section III" and so on.

**Contents**

0. [A crash course in reading Rust](#0-a-crash-course-in-reading-rust)
1. [The big picture](#1-the-big-picture)
2. [Quantum basics you need](#2-quantum-basics-you-need)
3. [Pauli operators](#3-pauli-operators)
4. [Stabilizers: describing a state with a few Paulis](#4-stabilizers-describing-a-state-with-a-few-paulis)
5. [Storing Paulis as bits](#5-storing-paulis-as-bits)
6. [Multiplying Paulis: `g` and `rowsum`](#6-multiplying-paulis-g-and-rowsum)
7. [The tableau](#7-the-tableau)
8. [Gates](#8-gates)
9. [Measurement](#9-measurement)
10. [The CHP language and `.chp` files](#10-the-chp-language-and-chp-files)
11. [The timing experiment (Figure 2)](#11-the-timing-experiment-figure-2)
12. [Counting stabilizer states (Proposition 2)](#12-counting-stabilizer-states-proposition-2)
13. [Arithmetic with bits: GF(2)](#13-arithmetic-with-bits-gf2)
14. [The canonical form (Section VI)](#14-the-canonical-form-section-vi)
15. [Inner product of two states](#15-inner-product-of-two-states)
16. [Section V: how hard is this problem?](#16-section-v-how-hard-is-this-problem)
17. [Mixed states (Section VII A)](#17-mixed-states-section-vii-a)
18. [Unusual starting states (Section VII B)](#18-unusual-starting-states-section-vii-b)
19. [A few non-stabilizer gates (Section VII C)](#19-a-few-non-stabilizer-gates-section-vii-c)
20. [How the code is organised](#20-how-the-code-is-organised)
21. [Using it from Python](#21-using-it-from-python)
22. [How we know it is correct (the tests)](#22-how-we-know-it-is-correct-the-tests)
23. [Where we differ from the paper](#23-where-we-differ-from-the-paper)
24. [Glossary](#24-glossary)

---

## 0. A crash course in reading Rust

You don't need to *write* Rust to follow this project, only to *read* it. This section covers every piece of syntax that appears in the code boxes, with examples taken from the real source. If you know Python, the comparisons in the tables should help.

### 0.1 How a Rust project is laid out

| File or folder | What it is | Python equivalent |
|---|---|---|
| `Cargo.toml` | project description: name, dependencies, settings | `pyproject.toml` |
| `src/lib.rs` | the library's root file; lists the modules | a package's `__init__.py` |
| `src/tableau.rs` etc. | one **module** per file | one `.py` module per file |
| `src/bin/chp.rs` | a separate runnable program | a script with `if __name__ == "__main__"` |
| `target/` | compiled output (ignore it) | `__pycache__` |

`src/lib.rs` contains lines like `pub mod tableau;`, which means "there is a module in `tableau.rs`, and it is public". Inside a module, `use crate::pauli::PauliString;` imports a name from another module of the same project (**crate** means "this project"). It is like `from .pauli import PauliString`.

Rust is **compiled**. `cargo build` turns the source into machine code, `cargo test` builds and runs the tests, and `cargo run --bin chp -- file.chp` runs the command-line program. Most mistakes (wrong types, a missing case) are caught at compile time, before anything runs.

### 0.2 Variables and types

```rust
let n = 5;               // an immutable variable: n cannot be changed later
let mut count = 0;       // `mut` makes it changeable
count += 1;
let s: i64 = 2 * n;      // a type can be written explicitly after a colon
let x = 3 as i64;        // `as` converts between number types
```

Variables are immutable unless declared `mut`. This is the opposite of Python, and it makes it easy to see which values a function changes.

Rust always knows each value's type. The ones used in this project:

| Type | Meaning | Example use here |
|---|---|---|
| `bool` | `true` / `false` | one bit of the tableau |
| `u8` | whole number 0..255 | a sign bit (0 or 1) |
| `i32`, `i64` | signed whole numbers (32 or 64 bits) | the value of `g`, a sum of `g` values |
| `u64` | unsigned 64-bit whole number | 64 packed qubit bits |
| `usize` | unsigned whole number used for sizes and indices | qubit numbers, row numbers |
| `f64` | floating-point number (Python's `float`) | probabilities |
| `Vec<T>` | growable list of `T` (Python `list`) | `Vec<u64>`: the packed words |
| `&[T]` | a borrowed view of part of a list (a **slice**) | one row's words, `&[u64]` |
| `String`, `&str` | owned text, borrowed text | Pauli strings like `"+XZ"` |
| `Option<T>` | either `Some(value)` or `None` | "the pivot row, if there is one" |
| `Result<T>` | either `Ok(value)` or `Err(error)` | anything that can fail |
| `(A, B)` | a tuple | `(word_index, mask)` |

A number can carry its type as a suffix: `0i64` is a zero of type `i64`, and `1u64` is a one of type `u64`.

### 0.3 Functions

```rust
pub fn g(x1: bool, z1: bool, x2: bool, z2: bool) -> i32 {
    ...
}
```

- `pub` means other modules may call it (public). Without `pub` it is private to its module. `pub(crate)` means "public inside this project, hidden from outside users".
- Each parameter has a name and a type: `x1: bool`.
- `-> i32` is the return type. No arrow means the function returns nothing.
- **The last expression is the return value**, as long as it has no semicolon. So a function whose body ends with `plus - minus` returns `plus - minus`. `return x;` also exists, for returning early.

### 0.4 Borrowing: `&` and `&mut`

Rust tracks who owns each piece of data. Passing data to a function usually **lends** it instead of copying it:

- `&value` lends a **read-only** view. The function may look but not change.
- `&mut value` lends a **writable** view. The function may change it. Only one writable loan can exist at a time, which rules out a whole class of bugs.
- `.clone()` makes a real copy, like Python's `copy.deepcopy`.

So `fn x_words(&self) -> &[u64]` returns a read-only view of the words without copying them, and `fn rowsum(&mut self, h: usize, i: usize)` is allowed to modify the tableau.

### 0.5 Structs and methods

A **struct** is a record of named fields, like a Python class with only data:

```rust
pub struct Tableau {
    n: usize,                 // number of qubits (private field)
    pub(crate) rows: Rows,    // the packed rows (visible inside this project)
}
```

Methods are attached with an `impl` block:

```rust
impl Tableau {
    pub fn new(n: usize) -> Self {               // no `self`: called as Tableau::new(3)
        assert!(n >= 1, "a tableau needs at least one qubit");
        let mut rows = Rows::new(n, 2 * n + 1);
        for q in 0..n {
            rows.set_x(q, q, true);              // destabilizer q = X_q
            rows.set_z(n + q, q, true);          // stabilizer q = Z_q
        }
        Tableau { n, rows }                      // build and return the struct
    }

    pub fn num_qubits(&self) -> usize {          // `&self`: reads the tableau
        self.n
    }
}
```

- `Self` means "the type this `impl` is for", here `Tableau`.
- A function with no `self` parameter is called with `::`, as in `Tableau::new(3)`. Python would use a classmethod for this.
- A method with `&self` or `&mut self` is called with a dot: `t.num_qubits()`. `self` is Python's `self`.
- `Tableau { n, rows }` is shorthand for `Tableau { n: n, rows: rows }`.

### 0.6 Enums and `match`

An **enum** is a type that is exactly one of several variants, and each variant can carry data:

```rust
pub enum Gate {
    Cnot(usize, usize),   // control, target
    H(usize),
    P(usize),
    Measure(usize),
}
```

`match` picks the branch that fits and unpacks the data. Every possible variant must be handled, or the code doesn't compile:

```rust
match *gate {
    Gate::Cnot(a, b) => self.cnot(a, b),   // binds a and b
    Gate::H(a) => self.h(a),
    Gate::P(a) => self.p(a),
    Gate::Measure(_) => value_err("expected a unitary gate"),   // `_` = "don't care"
}
```

The `*` in `*gate` reads the value behind a reference (`gate` is a `&Gate`).

`Option<T>` is a built-in enum: `Some(value)` or `None`. Two compact ways to unpack it:

```rust
if let Some(p) = self.random_pivot(a) { /* p is usable here */ }

let Some(p) = maybe_p else { return ...; };   // continue only if it is Some
```

### 0.7 Errors: `Result` and the `?` operator

Functions that can fail return `Result<T>`, which is either `Ok(value)` or `Err(error)`. The project's error type has two kinds, which become Python exceptions:

```rust
pub enum Error {
    Index(String),   // becomes IndexError in Python
    Value(String),   // becomes ValueError
}
```

The **`?` operator** means "if this failed, stop and pass the error up to my caller; otherwise give me the value". It replaces a lot of `if error: return error` boilerplate:

```rust
pub fn h(&mut self, a: usize) -> Result<()> {
    self.check_qubit(a)?;    // returns early with an IndexError if a is out of range
    self.rows.h(a);
    Ok(())                   // `()` means "nothing"; this is "success, no value"
}
```

`.map_err(...)` converts one kind of error into another. `unwrap()`, `expect("...")` and `assert!(...)` crash the program (a **panic**) if something is wrong. They are used only where failure would mean a bug, and in tests.

### 0.8 Loops, ranges, iterators and closures

```rust
for i in 0..n { ... }        // i = 0, 1, …, n−1  (Python: range(n))
for i in 0..=n { ... }       // i = 0, 1, …, n    (end included)
```

A **closure** is a small anonymous function, like Python's `lambda`. `|p| p + 1` takes `p` and returns `p + 1`. Closures are often passed to **iterator** methods:

```rust
(self.n..2 * self.n).find(|&p| self.rows.x(p, a))
// Python: next((p for p in range(n, 2*n) if rows.x(p, a)), None)
```

`find` returns an `Option`: the first `p` that passes the test, or `None`. Other iterator methods in the code: `.map(f)` (apply f to each item), `.filter(f)` (keep items where f is true), `.any(f)`, `.count()`, `.sum()`, and `.collect()` (gather the results into a `Vec` or similar).

A parameter written `choose: impl FnOnce() -> bool` means "any closure that takes nothing and returns a `bool`". The measurement code uses it to receive "how to pick the random outcome" (Section 9.5).

### 0.9 Bit operations

The tableau is all bits, so these operators appear everywhere:

| Rust | Python | Meaning |
|---|---|---|
| `a & b` | `a & b` | AND, bit by bit |
| `a \| b` | `a \| b` | OR |
| `a ^ b` | `a ^ b` | XOR |
| `!a` | `~a` | NOT (flip every bit). Careful: Python uses `~`, Rust uses `!` |
| `1u64 << k` | `1 << k` | a word with only bit k set |
| `a >> k` | `a >> k` | shift right by k |
| `a.count_ones()` | `bin(a).count("1")` | popcount |
| `x ^= y` | `x ^= y` | XOR into x |

The code finds a qubit's bit with this helper:

```rust
pub(crate) fn locate(q: usize) -> (usize, u64) {
    (q / WORD_BITS, 1u64 << (q % WORD_BITS))   // (which word, mask for the bit)
}
```

The **mask** is a word with a single 1 at the qubit's position. Then:

- `word & mask != 0` reads the bit,
- `word |= mask` sets it to 1,
- `word &= !mask` clears it to 0,
- `word ^= mask` flips it.

### 0.10 Macros, attributes and comments

- Names ending in `!` are **macros**, which are code generators: `println!("{x}")` prints, `format!(...)` builds a string (like an f-string), `vec![0; 4]` makes `[0, 0, 0, 0]`, and `assert!` / `assert_eq!` check conditions.
- Lines starting with `#[...]` are **attributes**, which are labels for the compiler:
  - `#[derive(Clone, Debug, PartialEq)]` automatically writes `.clone()`, debug printing and `==` for a type.
  - `#[cfg(test)] mod tests { ... }` is a block compiled only by `cargo test`, and each `#[test]` function inside it is one test.
  - `#[pyclass]`, `#[pymethods]` and `#[pyfunction]` come from pyo3 and expose Rust to Python (Section 20).
- `//` starts a comment. `///` is a documentation comment for the item below it, and `//!` documents the whole file.

### 0.11 Traits, briefly

A **trait** is a set of methods a type can promise to have, like an interface or a Python protocol. The project uses a few standard ones:

- `Display` gives a type a text form, so `to_string()` and `println!("{t}")` work. Python's equivalent is `__str__`. The code has `impl fmt::Display for Tableau { ... }`.
- `PartialEq` makes `==` work (`__eq__`).
- `Clone` makes `.clone()` work.

`fn determinate_outcome_with<O: Oracle>(...)` is a **generic** function: it works for any type `O` that implements the `Oracle` trait (Section 16). Likewise `Vec<T>` and `HashMap<K, V>` are generic containers holding any element type.

---

## 1. The big picture

A **quantum computer** works on **qubits** instead of bits. Simulating one on an ordinary computer is normally very expensive: describing n qubits exactly takes 2ⁿ numbers. For 50 qubits that is about 10¹⁵ numbers, more than any laptop can hold.

There is an important exception. If a circuit uses only three particular gates (**CNOT**, **Hadamard** and **Phase**) plus measurements, it can be simulated quickly, using only about n² bits. This is the **Gottesman–Knill theorem**, and such circuits are called **stabilizer circuits**. They matter in practice because quantum error correction is built almost entirely from them.

The paper makes that simulation faster, and then extends it:

| Paper section | What it contributes | Where in this project |
|---|---|---|
| III | A faster simulation algorithm (the "tableau algorithm") | `tableau.rs`, `rows.rs`, `pauli.rs` |
| III (end) | How similar two states are (inner product) | `inner_product.rs` |
| IV | A program called CHP, its input language, and speed measurements | `chp.rs`, `circuit.rs`, `experiment.rs`, `src/bin/chp.rs` |
| V | A proof about how hard the problem is | `parity_l.rs` |
| VI | Any such circuit can be rewritten in a standard 11-layer shape | `canonical.rs`, `pmh.rs` |
| VII A | Simulating "mixed" (partly random) states | `mixed.rs` |
| VII B | Starting from arbitrary product states | `nonstab_init.rs` |
| VII C | Allowing a few extra, non-stabilizer gates | `nonstab_gates.rs` |
| II, Prop. 2 | How many stabilizer states exist | `counting.rs` |

All of it is written in **Rust** (for speed) and made available to **Python** (for convenience). Section 20 explains how those two parts connect.

**Indexing.** The paper counts qubits and rows from 1. The code counts from 0, so "qubit 0" is the first qubit.

---

## 2. Quantum basics you need

### 2.1 One qubit

A classical bit is 0 or 1. A qubit's state is written as

```
α|0⟩ + β|1⟩
```

where α and β are complex numbers with |α|² + |β|² = 1. The symbols |0⟩ and |1⟩ are just labels for the two basic states. You can think of the state as the vector (α, β).

- **Measuring** the qubit gives 0 with probability |α|² and 1 with probability |β|². Afterwards the qubit *becomes* |0⟩ or |1⟩, whichever was observed.
- Some states you will see often:

| Name | State | Measuring gives |
|---|---|---|
| \|0⟩ | (1, 0) | always 0 |
| \|1⟩ | (0, 1) | always 1 |
| \|+⟩ | (1, 1)/√2 | 0 or 1, 50/50 |
| \|−⟩ | (1, −1)/√2 | 0 or 1, 50/50 |

### 2.2 Gates are matrices

A **gate** changes the state, and mathematically it is a matrix multiplying the state vector. The three gates this project cares about are:

```
Hadamard  H = 1/√2 [1  1]        Phase  P = [1 0]
                   [1 −1]                   [0 i]
```

- **H** turns |0⟩ into |+⟩ and |1⟩ into |−⟩. It "creates a superposition".
- **P** (also called **S**) leaves |0⟩ alone and multiplies the |1⟩ part by i. In the Python API it is `apply_s`.
- **CNOT** acts on two qubits, a *control* and a *target*. It flips the target if the control is 1: |a, b⟩ → |a, b XOR a⟩.

### 2.3 Many qubits, and why simulation is usually hard

n qubits have 2ⁿ basic states (|00…0⟩ to |11…1⟩), and a general state needs one complex number for each. That is why ordinary simulators run out of memory quickly. The trick in this project is to avoid storing those 2ⁿ numbers.

### 2.4 Entanglement, in one example

Start with |00⟩, apply H to qubit 0, then CNOT from qubit 0 to qubit 1:

```
|00⟩  --H on qubit 0-->  (|00⟩ + |10⟩)/√2  --CNOT 0→1-->  (|00⟩ + |11⟩)/√2
```

This is the **Bell state**. Measuring either qubit gives a random bit, but both qubits always give the *same* bit. The tests use this example over and over.

---

## 3. Pauli operators

### 3.1 The four Pauli matrices

```
I = [1 0]   X = [0 1]   Y = [0 −i]   Z = [1  0]
    [0 1]       [1 0]       [i  0]       [0 −1]
```

- **I** does nothing.
- **X** flips the bit: |0⟩ ↔ |1⟩ (a quantum NOT).
- **Z** leaves |0⟩ alone and multiplies |1⟩ by −1.
- **Y** does both, with an extra factor of i.

### 3.2 Multiplying them

Multiplying two Paulis always gives another Pauli, possibly times a factor from {1, i, −1, −i}:

| left × right | I | X | Y | Z |
|---|---|---|---|---|
| **I** | I | X | Y | Z |
| **X** | X | I | iZ | −iY |
| **Y** | Y | −iX | I | iX |
| **Z** | Z | iY | −iX | I |

A memory aid: arrange them in a circle X → Y → Z → X. Multiplying "forwards" around the circle (XY, YZ, ZX) gives +i times the third one. Multiplying "backwards" (YX, ZY, XZ) gives −i. Any Pauli times itself is I.

### 3.3 Commute or anticommute

Two matrices **commute** if AB = BA. Every pair of Paulis either commutes (AB = BA) or **anticommutes** (AB = −BA):

- the same letter twice, or anything with I, commutes;
- two *different* non-identity letters anticommute (for example XZ = −ZX).

### 3.4 Paulis on several qubits

On n qubits, a Pauli is one letter per qubit, plus a sign, for example `+XIZ` or `−YY`. It means "apply X to qubit 0, nothing to qubit 1, Z to qubit 2", and the sign multiplies everything. The leftmost letter belongs to qubit 0.

- **Multiplying** two of them is done qubit by qubit, and the per-qubit factors multiply together. For example XX × ZZ = (XZ)(XZ) = (−iY)(−iY) = (−i)²·YY = −YY.
- **Commuting:** two multi-qubit Paulis commute if they anticommute on an **even** number of qubits, and anticommute if that number is odd. Each anticommuting position contributes a factor of −1, so an even number of them cancels. Example: XX and ZZ anticommute on both qubits, which is 2 positions, so they commute.

> **Derivation: why "even number of anticommuting positions" means commute**
>
> Write P = P₁ ⊗ P₂ ⊗ … ⊗ Pₙ and Q = Q₁ ⊗ … ⊗ Qₙ. Tensor products multiply position by position:
>
> ```
> PQ = (P₁Q₁) ⊗ (P₂Q₂) ⊗ … ⊗ (PₙQₙ)
> ```
>
> At each position, single-qubit Paulis either commute or anticommute, so PⱼQⱼ = εⱼ QⱼPⱼ with εⱼ = +1 or −1. Substituting:
>
> ```
> PQ = (ε₁Q₁P₁) ⊗ … ⊗ (εₙQₙPₙ) = (ε₁ε₂⋯εₙ) · (Q₁P₁ ⊗ … ⊗ QₙPₙ) = (ε₁ε₂⋯εₙ) · QP
> ```
>
> The numbers εⱼ can be pulled out in front because they are just scalars. The product ε₁ε₂⋯εₙ is −1 exactly when an **odd** number of the εⱼ are −1. ∎

---

## 4. Stabilizers: describing a state with a few Paulis

### 4.1 What "stabilizes" means

An operator P **stabilizes** a state |ψ⟩ if P|ψ⟩ = |ψ⟩: applying P changes nothing. Examples:

| Pauli | State it stabilizes |
|---|---|
| +Z | \|0⟩ |
| −Z | \|1⟩ |
| +X | \|+⟩ |
| −X | \|−⟩ |

So "+Z" is a complete description of |0⟩: it is the only state that +Z leaves unchanged.

### 4.2 The key idea

For n qubits, a special family of states, the **stabilizer states**, are each described exactly by n Paulis. Those Paulis must:

1. commute with each other,
2. be independent (none is a product of the others),
3. have sign + or − (not ±i).

They are called the **generators**, and every product of generators also stabilizes the state. Examples:

| State | Generators |
|---|---|
| \|00⟩ | +ZI, +IZ |
| \|+0⟩ | +XI, +IZ |
| Bell state (\|00⟩+\|11⟩)/√2 | +XX, +ZZ |

Check the Bell state: XX swaps |00⟩ ↔ |11⟩, so it leaves the sum unchanged. ZZ multiplies both |00⟩ and |11⟩ by (+1)(+1) or (−1)(−1) = +1, so it also leaves it unchanged.

n generators of n qubits each take about 2n² bits, instead of 2ⁿ complex numbers. That is the whole saving.

### 4.3 Why the three gates keep us inside this world

Suppose P stabilizes |ψ⟩ and we apply a gate U. Then (U P U†) stabilizes the new state U|ψ⟩, where U† is U's inverse:

```
(U P U†)(U|ψ⟩) = U P |ψ⟩ = U|ψ⟩
```

For H, P and CNOT, the operator U P U† of a Pauli is *again a Pauli*. So after each gate we only need to update each generator. These are the rules ("how a gate transforms each Pauli"):

| Gate | X becomes | Z becomes | Y becomes |
|---|---|---|---|
| H | Z | X | −Y |
| P | Y | Z | −X |

| CNOT (control c, target t) | becomes |
|---|---|
| X on c | X on c and X on t |
| Z on t | Z on c and Z on t |
| X on t, Z on c | unchanged |

> **Derivation: the H and P rows of the table**
>
> *Hadamard.* Multiply the matrices out:
>
> ```
> HX = 1/√2 [1  1] [0 1] = 1/√2 [ 1 1]
>           [1 −1] [1 0]        [−1 1]
>
> HXH = 1/2 [ 1 1] [1  1] = 1/2 [2  0] = [1  0] = Z
>           [−1 1] [1 −1]       [0 −2]   [0 −1]
> ```
>
> For Z, use H·H = I (applying H twice does nothing): HZH = H(HXH)H = (HH)X(HH) = X.
>
> For Y, use Y = iXZ (check with the multiplication table: XZ = −iY, so iXZ = −i²Y = Y). Inserting HH = I between X and Z:
>
> ```
> HYH = i·H X Z H = i·(HXH)(HZH) = i·Z·X = i·(iY) = −Y
> ```
>
> *Phase.* P = diag(1, i) and P† = diag(1, −i):
>
> ```
> P X P† = [1 0] [0 1] [1  0] = [0 1] [1  0] = [0 −i] = Y
>          [0 i] [1 0] [0 −i]   [i 0] [0 −i]   [i  0]
> ```
>
> P Z P† = Z because diagonal matrices commute. Then, with P†P = I inserted:
>
> ```
> P Y P† = i·(P X P†)(P Z P†) = i·Y·Z = i·(iX) = −X
> ```
>
> The same trick, "write Y as iXZ and transform X and Z separately", works for any of these gates. That is why the tables only need X and Z; Y follows.

> **Derivation: the CNOT signs**
>
> CNOT's two basic rules are standard: an X on the control is copied to the target (X_c → X_c X_t), and a Z on the target is copied back to the control (Z_t → Z_c Z_t). X_t and Z_c are unchanged. A minus sign can only appear when Y's are involved, through Y = iXZ. Here is the hardest case, Y on both qubits:
>
> ```
> Y_c Y_t = (i X_c Z_c)(i X_t Z_t) = − X_c Z_c X_t Z_t          (i·i = −1)
>
> apply the rules:  X_c → X_c X_t,  Z_c → Z_c,  X_t → X_t,  Z_t → Z_c Z_t
>
>          → − (X_c X_t)(Z_c)(X_t)(Z_c Z_t)
> ```
>
> Operators on different qubits commute, so collect each qubit's factors, keeping their order:
>
> ```
> control:  X_c Z_c Z_c = X_c        (Z_c Z_c = I)
> target:   X_t X_t Z_t = Z_t        (X_t X_t = I)
> ```
>
> Result: Y⊗Y → **−** X⊗Z. Doing the same for the other cases where the control has an X part and the target has a Z part gives:
>
> | before | after | sign flipped? |
> |---|---|---|
> | X ⊗ Z | −Y ⊗ Y | yes |
> | X ⊗ Y | +Y ⊗ Z | no |
> | Y ⊗ Z | +X ⊗ Y | no |
> | Y ⊗ Y | −X ⊗ Z | yes |
>
> In every other combination the rules never put two anticommuting factors in the wrong order, so no sign appears. Section 8 turns this table into a one-line bit formula.

Bell example, written with generators only:

```
start      +ZI  +IZ          (state |00⟩)
H on 0     +XI  +IZ          (Z→X on qubit 0)
CNOT 0→1   +XX  +ZZ          (X on control spreads; Z on target spreads)
```

The result is exactly the Bell state's generators. Gates other than these three (for example the T gate) do **not** map Paulis to Paulis, which is why Section 19 needs extra machinery for them.

---

## 5. Storing Paulis as bits

### 5.1 Two bits per qubit

Each single-qubit Pauli is stored as two bits, called x and z:

| Pauli | x | z |
|---|---|---|
| I | 0 | 0 |
| X | 1 | 0 |
| Z | 0 | 1 |
| Y | 1 | 1 |

Think of x as "has an X part" and z as "has a Z part"; Y has both. The sign is stored separately as one bit **r**: r = 0 means +, r = 1 means −.

The useful property: **multiplying two Paulis is just XOR of their bits, apart from the sign.** XOR ("exclusive or", written ⊕) gives 1 when exactly one input is 1. For example X × Z: (1,0) ⊕ (0,1) = (1,1) = Y, and the table says XZ = −iY, so the letter is right. Only the factor (−i) needs separate work; Section 6 handles it.

### 5.2 Commuting, computed from bits

Two Paulis anticommute on a qubit exactly when x₁·z₂ ⊕ z₁·x₂ = 1 there. Adding this up (with XOR) over all qubits gives the **symplectic product**: 0 means the multi-qubit Paulis commute, 1 means they anticommute. The code uses this check constantly.

> **Derivation: why x₁z₂ ⊕ z₁x₂ detects anticommuting**
>
> Just try every pair. Rows are the first Pauli, columns the second, and each entry is x₁z₂ ⊕ z₁x₂:
>
> | | I (0,0) | X (1,0) | Z (0,1) | Y (1,1) |
> |---|---|---|---|---|
> | **I (0,0)** | 0 | 0 | 0 | 0 |
> | **X (1,0)** | 0 | 0⊕0 = 0 | 1⊕0 = **1** | 1⊕0 = **1** |
> | **Z (0,1)** | 0 | 0⊕1 = **1** | 0⊕0 = 0 | 0⊕1 = **1** |
> | **Y (1,1)** | 0 | 0⊕1 = **1** | 1⊕0 = **1** | 1⊕1 = 0 |
>
> The 1s are exactly the six pairs of *different, non-identity* letters, which are the anticommuting pairs (Section 3.3). For n qubits, XOR-ing these bits over all positions gives 1 when the number of anticommuting positions is odd, which by the Section 3.4 derivation is exactly when the whole Paulis anticommute. ∎

> **Reading the code:** `anticommute_words` in `pauli.rs`
>
> ```rust
> pub fn anticommute_words(x1: &[u64], z1: &[u64], x2: &[u64], z2: &[u64]) -> bool {
>     let mut acc = 0u64;                         // running XOR, starts at all-zero bits
>     for k in 0..x1.len() {                      // for each 64-qubit word
>         acc ^= (x1[k] & z2[k]) ^ (z1[k] & x2[k]);   // x1·z2 ⊕ z1·x2 for 64 qubits at once
>     }
>     acc.count_ones() % 2 == 1                   // odd number of 1s → anticommute
> }
> ```
>
> - The four inputs are `&[u64]`, read-only views of the packed words (Section 0.4).
> - `x1.len()` is the number of words. `x1[k]` is the k-th word, indexed like Python.
> - `&` and `^` act on all 64 bits at once (Section 0.9), so one loop step handles 64 qubits.
> - The last line has no semicolon, so it is the return value (Section 0.3). `%` is remainder, as in Python.

### 5.3 Packing 64 qubits into one number

A computer handles 64-bit integers (`u64` in Rust) in one step. So the x-bits of 64 qubits are packed into one integer, and likewise the z-bits. Qubit q goes in word `q / 64`, at bit position `q % 64`.

Why bother? XORing two whole rows of 100 qubits is then 2 integer operations instead of 100. Counting 1 bits ("**popcount**") is a single CPU instruction too. Unused bits above n in the last word ("padding") are always kept at 0, so they never get counted by mistake. A Rust test checks this.

`pauli.rs` defines `PauliString` (one Pauli on n qubits: x-words, z-words, and a phase 0..3 meaning +, +i, −, −i). It can parse and print strings like `"+XIZ"` and `"-iY"`, multiply, and test commuting.

---

## 6. Multiplying Paulis: `g` and `rowsum`

### 6.1 The function g

When we multiply single-qubit Paulis P₁ × P₂ = iᵉ × P₃, the exponent e is −1, 0 or +1. The paper gives a formula for it, `g(x1, z1, x2, z2)`, which is a compact version of the multiplication table:

| P₁ is | g = |
|---|---|
| I | 0 |
| Y | z₂ − x₂ |
| X | z₂ × (2x₂ − 1) |
| Z | x₂ × (1 − 2z₂) |

Check one case: X × Z. P₁ = X, and P₂ = Z has x₂ = 0, z₂ = 1, so g = 1 × (0 − 1) = −1, giving XZ = i⁻¹·Y = −iY. ✓

> **Derivation: where the three formulas come from**
>
> Each formula is a compact way to write one row of the multiplication table (Section 3.2). Take **P₁ = X**. Reading the X row of the table, the exponents are:
>
> | P₂ | (x₂, z₂) | X·P₂ | exponent wanted |
> |---|---|---|---|
> | I | (0,0) | X | 0 |
> | X | (1,0) | I | 0 |
> | Y | (1,1) | iZ | +1 |
> | Z | (0,1) | −iY | −1 |
>
> The exponent is 0 whenever z₂ = 0, so it has the form z₂ × (something). When z₂ = 1 it must be +1 for x₂ = 1 and −1 for x₂ = 0. The expression 2x₂ − 1 does exactly that: it maps 1 → +1 and 0 → −1. Hence **g = z₂(2x₂ − 1)**.
>
> **P₁ = Z** works the same way with the roles of x and z swapped. The exponent is nonzero only when x₂ = 1, giving +1 for X (z₂ = 0) and −1 for Y (z₂ = 1). So g = x₂(1 − 2z₂).
>
> **P₁ = Y**: Y·X = −iZ (−1), Y·Z = iX (+1), Y·I = Y (0) and Y·Y = I (0). The pattern "−1 when only x₂ = 1, +1 when only z₂ = 1, 0 when both or neither" is exactly z₂ − x₂.
>
> **P₁ = I** never produces a phase, so g = 0.

> **Reading the code:** `g` in `pauli.rs`
>
> ```rust
> pub fn g(x1: bool, z1: bool, x2: bool, z2: bool) -> i32 {
>     let (x2, z2) = (x2 as i32, z2 as i32);   // turn true/false into 1/0
>     match (x1, z1) {                          // look at P₁'s two bits together
>         (false, false) => 0,                  // I
>         (true, true) => z2 - x2,              // Y
>         (true, false) => z2 * (2 * x2 - 1),   // X
>         (false, true) => x2 * (1 - 2 * z2),   // Z
>     }
> }
> ```
>
> - `let (x2, z2) = (...)` unpacks a tuple. It also **shadows** the old `x2` and `z2`: new variables with the same names replace the `bool` ones from here on.
> - `match` on the tuple `(x1, z1)` picks one of four branches (Section 0.6). The compiler checks that all four `bool` combinations are covered.
> - The `match` is the last expression, so whichever branch runs gives the return value.

### 6.2 Doing all qubits at once

Looping over qubits one by one is slow. Instead, `g_sum` notices that g is +1 in exactly three bit patterns (Y·Z, X·Y, Z·X) and −1 in three others. One combination of AND/NOT operations per sign picks out every qubit matching those patterns, 64 qubits at a time. The total is then (number of +1 qubits) − (number of −1 qubits), using popcount.

> **Reading the code:** `g_sum` in `pauli.rs`
>
> ```rust
> pub fn g_sum(x1: &[u64], z1: &[u64], x2: &[u64], z2: &[u64]) -> i64 {
>     let mut plus = 0i64;
>     let mut minus = 0i64;
>     for k in 0..x1.len() {
>         let (a, b, c, d) = (x1[k], z1[k], x2[k], z2[k]);
>         let p = (a & b & !c & d) | (a & !b & c & d) | (!a & b & c & !d);
>         let m = (a & b & c & !d) | (a & !b & !c & d) | (!a & b & c & d);
>         plus += p.count_ones() as i64;
>         minus += m.count_ones() as i64;
>     }
>     plus - minus
> }
> ```
>
> Read each bracket as a pattern test. `a & b & !c & d` is 1 at a qubit exactly when x₁ = 1, z₁ = 1 (P₁ = Y), x₂ = 0 and z₂ = 1 (P₂ = Z), i.e. the case Y·Z, where g = +1. `!c` means "c must be 0". The three `+1` patterns are OR-ed into `p`, and the three `−1` patterns into `m`.
>
> Why padding bits never count: each pattern contains at least one un-negated input (like `d`), and that input is 0 in the padding, so the whole pattern is 0 there.

### 6.3 rowsum(h, i)

`rowsum` is the paper's name for "replace row h by row i × row h", keeping the sign correct. The sign comes from adding up the exponent of i:

```
total = 2·(sign bit of row h) + 2·(sign bit of row i) + Σ g over all qubits
```

using the fact that −1 = i². Only the remainder modulo 4 matters, because i⁴ = 1:

| total mod 4 | overall factor | new sign |
|---|---|---|
| 0 | +1 | + |
| 2 | −1 | − |
| 1 or 3 | ±i | not allowed |

A factor of ±i happens only if the two rows anticommute. The algorithm never asks for that, so if it happens the code reports an error and leaves the row unchanged, instead of storing a wrong answer.

> **Derivation: the rowsum formula**
>
> Write row i as (−1)^{rᵢ}·P and row h as (−1)^{r_h}·Q, where P and Q are the letter parts and rᵢ, r_h are sign bits.
>
> 1. Multiply, pulling the signs to the front (they are just numbers):
>    `row_i · row_h = (−1)^{rᵢ + r_h} · PQ`
> 2. Multiply the letter parts position by position. At each qubit j, Section 6.1 gives Pⱼ Qⱼ = i^{gⱼ} Rⱼ, where Rⱼ is the XOR letter. So
>    `PQ = i^{g₁} R₁ ⊗ … ⊗ i^{gₙ} Rₙ = i^{g₁+…+gₙ} · R`
> 3. Rewrite the sign as a power of i, using −1 = i²: `(−1)^{rᵢ + r_h} = i^{2rᵢ + 2r_h}`
> 4. Combine the powers of i (exponents add):
>    `row_i · row_h = i^{2rᵢ + 2r_h + Σⱼ gⱼ} · R`
> 5. Powers of i repeat every 4 (i⁴ = 1), so only the exponent mod 4 matters. Exponent 0 means +1, so the new sign bit is 0; exponent 2 means −1, so the new sign bit is 1. That is the table above.
>
> Why an odd exponent means anticommuting: if P and Q commute, (PQ)† = Q†P† = QP = PQ, so PQ is Hermitian, and a Hermitian Pauli can only carry ±1, which is an even exponent. If they anticommute, (PQ)† = QP = −PQ, which forces a factor of ±i, an odd exponent. ∎

> **Reading the code:** `Rows::rowsum` in `rows.rs`
>
> ```rust
> pub fn rowsum(&mut self, h: usize, i: usize) -> Result<(), u8> {
>     let w = self.w;                                  // words per row
>     let (hs, is) = (h * w, i * w);                   // where rows h and i start
>     let mut s = 2 * (self.r[h] as i64 + self.r[i] as i64);    // 2r_h + 2r_i
>     s += g_sum(
>         &self.x[is..is + w],                         // row i's x-words (a slice)
>         &self.z[is..is + w],
>         &self.x[hs..hs + w],                         // row h's x-words
>         &self.z[hs..hs + w],
>     );
>     let s = s.rem_euclid(4);                         // mod 4, always in 0..4
>     if s % 2 == 1 {
>         return Err(s as u8);                         // odd: rows anticommute
>     }
>     self.r[h] = (s / 2) as u8;                       // 0 → sign +, 2 → sign −
>     for k in 0..w {
>         self.x[hs + k] ^= self.x[is + k];            // letters: XOR row i into row h
>         self.z[hs + k] ^= self.z[is + k];
>     }
>     Ok(())
> }
> ```
>
> - All rows live in one long `Vec<u64>`; row `h` occupies positions `h*w .. h*w + w`. `&self.x[is..is + w]` is a **slice** of that range (Section 0.2), like `x[is:is+w]` in Python but without copying.
> - `Result<(), u8>` means "success with no value, or failure carrying the bad sum". The caller turns that into a proper error message.
> - `rem_euclid(4)` is a remainder that is never negative, like Python's `%`. Rust's own `%` can return a negative number for a negative input, and Σg can be negative.
> - The sign is checked *before* anything is written, so a failed rowsum leaves row h untouched.

**Worked example.** Row i = +XZ, row h = +ZX.
- Qubit 0: X × Z → g = −1.
- Qubit 1: Z × X → g = +1.
- Total = 0 + 0 − 1 + 1 = 0, so the sign is +.
- Bits: x = (1,0) ⊕ (0,1) = (1,1), z = (0,1) ⊕ (1,0) = (1,1), so both qubits are Y.

Result: +YY. Check by hand: (XZ)(ZX) per qubit is (−iY)(iY) = −i·i·YY = +YY. ✓

---

## 7. The tableau

### 7.1 What is stored

For n qubits the paper stores a table (the **tableau**) of 2n rows, each a signed Pauli, plus one spare row:

```
rows 0 .. n−1     destabilizers
rows n .. 2n−1    stabilizers (the generators from Section 4)
row  2n           scratch space for calculations
```

### 7.2 What destabilizers are and why they are there

The stabilizers alone describe the state. The **destabilizers** are extra helper rows, one per stabilizer, chosen so that:

- destabilizer i **anticommutes** with stabilizer i,
- destabilizer i **commutes** with every other stabilizer,
- all destabilizers commute with each other.

They cost double the memory, but they make measurement fast. Without them, finding a measurement result means solving a system of equations, which takes about n³ steps. With them it takes about n² (Section 9). That is the paper's main improvement. The rules above are the paper's **Proposition 3**, and `check_invariants()` verifies them.

### 7.3 Starting state

The simulation starts in |00…0⟩. Its stabilizers are Z on each qubit. A matching destabilizer for "Z on qubit q" is "X on qubit q", since X and Z anticommute on that qubit and touch nothing else. For n = 3, `print(Tableau(3))` shows:

```
+XII      ← destabilizers
+IXI
+IIX
----
+ZII      ← stabilizers
+IZI
+IIZ
```

### 7.4 The bits behind it

The paper draws this as a binary matrix: each row has n x-bits, n z-bits and a sign bit r. In the code (`rows.rs`) each row's x-bits and z-bits are packed into 64-bit words as in Section 5.3, and rows are stored one after another in memory.

> **Reading the code:** the `Rows` struct in `rows.rs`
>
> ```rust
> #[derive(Clone, Debug)]
> pub struct Rows {
>     pub(crate) nq: usize,      // number of qubits
>     pub(crate) nrows: usize,   // number of rows (2n + 1 for a tableau)
>     pub(crate) w: usize,       // 64-bit words per row = ceil(nq / 64)
>     pub(crate) x: Vec<u64>,    // all x-bits: row 0's words, then row 1's, …
>     pub(crate) z: Vec<u64>,    // all z-bits, same layout
>     pub(crate) r: Vec<u8>,     // one sign per row: 0 = +, 1 = −
> }
> ```
>
> - `#[derive(Clone, Debug)]` asks the compiler to write `.clone()` and debug printing automatically (Section 0.10).
> - Bit x of row i, qubit q, is in word `x[i * w + q / 64]` at bit position `q % 64`. The accessor `rows.x(i, q)` does that arithmetic and returns a `bool`.
> - `Tableau` (in `tableau.rs`) is a thin wrapper: a `Rows` with 2n + 1 rows, plus the qubit count, plus methods that check their arguments before touching the bits. Its `new` is shown in Section 0.5.

---

## 8. Gates

### 8.1 The update rules

A gate updates *every* row of the tableau, using only the bits for the qubits it touches. These are the bit versions of the table in Section 4.3, straight from the paper. "Row i" means every row in turn, and ⊕= means "XOR into".

**Hadamard on qubit a:**
- sign: rᵢ ⊕= xᵢₐ·zᵢₐ (only a Y on qubit a flips the sign, since HYH = −Y)
- swap xᵢₐ and zᵢₐ (X ↔ Z)

**Phase on qubit a:**
- sign: rᵢ ⊕= xᵢₐ·zᵢₐ (a Y becomes −X)
- zᵢₐ ⊕= xᵢₐ (X becomes Y, Y becomes X, Z stays Z)

**CNOT from a (control) to b (target):**
- sign: rᵢ ⊕= xᵢₐ·zᵢᵦ·(xᵢᵦ ⊕ zᵢₐ ⊕ 1)
- xᵢᵦ ⊕= xᵢₐ (an X on the control spreads to the target)
- zᵢₐ ⊕= zᵢᵦ (a Z on the target spreads to the control)

Each gate looks at one word per row, so it costs about n operations.

> **Derivation: from the Section 4.3 tables to these bit rules**
>
> *Hadamard.* The table says X ↔ Z, and Y → −Y. In bits, X = (1,0) and Z = (0,1), so "X ↔ Z" is "swap x and z". Swapping leaves Y = (1,1) and I = (0,0) unchanged, which is right for the letters. The only extra effect is Y's minus sign, and Y is exactly the case x = 1 **and** z = 1. So flip the sign when x·z = 1.
>
> *Phase.* The table says X → Y, Y → −X, Z → Z. Check the rule z ⊕= x on each (bits before → after):
>
> | before | z ⊕= x gives | correct letter? | x·z before | sign flip? |
> |---|---|---|---|---|
> | X (1,0) | (1,1) = Y | yes | 0 | no ✓ |
> | Y (1,1) | (1,0) = X | yes | 1 | yes ✓ (Y → −X) |
> | Z (0,1) | (0,1) = Z | yes | 0 | no ✓ |
> | I (0,0) | (0,0) = I | yes | 0 | no ✓ |
>
> The sign is computed from the bits *before* the update, which is why the code computes the sign first.
>
> *CNOT.* The letter rules are the two "copying" rules: an X on the control is XOR-ed into the target's x bit (x_b ⊕= x_a), and a Z on the target is XOR-ed into the control's z bit (z_a ⊕= z_b). For the sign, evaluate the paper's formula x_a·z_b·(x_b ⊕ z_a ⊕ 1) on the four cases from the CNOT table in Section 4.3:
>
> | control, target | x_a | z_b | x_b | z_a | x_b ⊕ z_a ⊕ 1 | formula | table says |
> |---|---|---|---|---|---|---|---|
> | X, Z | 1 | 1 | 0 | 0 | 1 | **1** | flip ✓ |
> | X, Y | 1 | 1 | 1 | 0 | 0 | 0 | no flip ✓ |
> | Y, Z | 1 | 1 | 0 | 1 | 0 | 0 | no flip ✓ |
> | Y, Y | 1 | 1 | 1 | 1 | 1 | **1** | flip ✓ |
>
> In every other case x_a·z_b = 0, so the formula gives 0, matching "no sign appears". The factor (x_b ⊕ z_a ⊕ 1) is 1 when x_b and z_a are equal, i.e. when the control and target are "both plain" (X, Z) or "both Y". ∎

> **Reading the code:** `Rows::h` in `rows.rs`
>
> ```rust
> pub fn h(&mut self, a: usize) {
>     let (k, m) = locate(a);                 // word index k and bit mask m of qubit a
>     let w = self.w;
>     for i in 0..self.nrows {                // every row
>         let xi = self.x[i * w + k];         // the word holding qubit a's x bit
>         let zi = self.z[i * w + k];         // … and its z bit
>         if xi & zi & m != 0 {               // both bits set at qubit a → a Y
>             self.r[i] ^= 1;                 // flip the sign
>         }
>         self.x[i * w + k] = (xi & !m) | (zi & m);   // new x bit = old z bit
>         self.z[i * w + k] = (zi & !m) | (xi & m);   // new z bit = old x bit
>     }
> }
> ```
>
> The two last lines swap a single bit inside a word. `xi & !m` keeps every bit of `xi` except qubit a's (the mask m has a 1 only there, so `!m` has a 0 only there). `zi & m` keeps *only* qubit a's bit from `zi`. OR-ing them puts z's bit into x's slot. The other 63 qubits in the word are untouched. `Rows::p` and `Rows::cnot` follow the same pattern.

### 8.2 Worked example: building the Bell state

Starting from the n = 2 identity tableau:

| row | start | after H on 0 | after CNOT 0→1 |
|---|---|---|---|
| destabilizer 0 | +XI | +ZI | +ZI |
| destabilizer 1 | +IX | +IX | +IX |
| stabilizer 0 | +ZI | +XI | **+XX** |
| stabilizer 1 | +IZ | +IZ | **+ZZ** |

The stabilizers end up as +XX and +ZZ, the Bell state from Section 4.2. No sign changed: H only flips a sign if the row has a Y on qubit 0, and none does.

---

## 9. Measurement

Measuring qubit a "in the standard basis" means asking "is it 0 or 1?". The possible results correspond to the Pauli Z on qubit a (written Zₐ): outcome 0 is "Zₐ = +1" and outcome 1 is "Zₐ = −1". There are two cases.

### 9.1 How to tell which case applies

Look at the stabilizer rows. If **any** stabilizer has an X or Y on qubit a (x bit = 1), it anticommutes with Zₐ, and the outcome is **random** (Case I). If none does, Zₐ commutes with the whole stabilizer, and the outcome is **already determined** (Case II). This check alone takes only n steps; it is `is_deterministic(a)`.

### 9.2 Case I: random outcome

Let p be the first stabilizer row with x = 1 on qubit a.

1. Every *other* row with x = 1 on qubit a gets multiplied by row p (`rowsum(i, p)`). This makes them all commute with Zₐ, so they stay valid after the measurement.
2. Copy row p into its destabilizer slot (row p − n). The old stabilizer becomes the new one's helper.
3. Replace row p by ±Zₐ, with the sign chosen by a fair coin flip. That sign is the measurement result.

(One small difference from the paper: it also multiplies row p − n by row p in step 1. That product would have a factor of ±i, and step 2 overwrites the row anyway, so the code skips it.)

> **Derivation: why Case I is exactly 50/50**
>
> Let S be the stabilizer in row p. It anticommutes with Zₐ (it has an X or Y on qubit a), and S|ψ⟩ = |ψ⟩. Taking the conjugate transpose of that, ⟨ψ|S = ⟨ψ| as well, because S is Hermitian. Now compute the expectation value of Zₐ in two ways:
>
> ```
> ⟨ψ|Zₐ|ψ⟩ = ⟨ψ|Zₐ S|ψ⟩          (insert S|ψ⟩ = |ψ⟩)
>          = −⟨ψ|S Zₐ|ψ⟩          (they anticommute: Zₐ S = −S Zₐ)
>          = −⟨ψ|Zₐ|ψ⟩            (use ⟨ψ|S = ⟨ψ|)
> ```
>
> A number equal to minus itself is 0, so ⟨Zₐ⟩ = 0. Since p(0) − p(1) = ⟨Zₐ⟩ and p(0) + p(1) = 1, we get p(0) = p(1) = ½. ∎

> **Derivation: why the three update steps give a valid tableau**
>
> After the measurement the state is stabilized by ±Zₐ, so ±Zₐ must replace one stabilizer. Every other row must keep the Proposition 3 relations with it.
>
> - *Step 1.* A row R with an X or Y on qubit a anticommutes with Zₐ, and so does S. Their product RS gets two minus signs when moved past Zₐ, so it **commutes** with Zₐ (the same counting as Section 3.4). Multiplying R by S also doesn't change which other rows R commutes with, apart from S's own partner, which step 2 overwrites anyway. After step 1, every row except S and its destabilizer commutes with Zₐ.
> - *Step 2.* The new stabilizer ±Zₐ needs a destabilizer that anticommutes with it and commutes with all the other stabilizers. The old S does exactly that: it anticommutes with Zₐ, and it commuted with the other stabilizers before (they were all stabilizers together, or became products with S in step 1). So S moves into the destabilizer slot.
> - *Step 3.* The other stabilizers still stabilize the post-measurement state, because they commute with Zₐ (step 1), and so they commute with the projector (I ± Zₐ)/2. Adding ±Zₐ, with the observed sign, completes the set.

### 9.3 Case II: determined outcome

Zₐ must equal ± a product of some stabilizers. The question is which ones, and what the sign is. Normally finding that product means solving equations (n³ work). The destabilizers give the answer directly: **stabilizer i is in the product exactly when destabilizer i has x = 1 on qubit a.** (Destabilizer i anticommutes with stabilizer i only, so it "detects" whether stabilizer i is involved.)

So: clear the scratch row, multiply in every stabilizer i whose destabilizer has x = 1 on qubit a, and read the scratch row's sign. The sign is the outcome.

> **Derivation: why "destabilizer i has x = 1 on qubit a" picks the right stabilizers**
>
> *Step 1: ±Zₐ really is a product of stabilizers.* In Case II, Zₐ commutes with every stabilizer Sⱼ. So Sⱼ(Zₐ|ψ⟩) = Zₐ Sⱼ|ψ⟩ = Zₐ|ψ⟩: the vector Zₐ|ψ⟩ is also stabilized by every Sⱼ. Only one state (up to a factor) is stabilized by all n generators, so Zₐ|ψ⟩ = λ|ψ⟩, and λ = ±1 because Zₐ² = I. Then ±Zₐ stabilizes |ψ⟩, and every Pauli that stabilizes a stabilizer state is a product of its generators. So
>
> ```
> ±Zₐ = ∏_{h ∈ J} S_h      for some set J of stabilizers
> ```
>
> *Step 2: test each destabilizer Dᵢ against both sides.* Ask whether Dᵢ commutes with Zₐ, and answer it twice:
>
> - **Directly:** Dᵢ anticommutes with Zₐ exactly when Dᵢ has an X or Y on qubit a, i.e. when its x bit at qubit a is 1.
> - **Through the product:** moving Dᵢ past ∏_{h∈J} S_h picks up one minus sign for each S_h it anticommutes with. By Proposition 3, Dᵢ anticommutes with Sᵢ only. So there is one minus sign if i ∈ J, and none otherwise.
>
> Both answers describe the same operator, so they agree: **i ∈ J exactly when Dᵢ has x = 1 on qubit a.** ∎
>
> This is the paper's formula cᵢ = Rᵢ · Zₐ = xᵢₐ, written out in words. It is also why destabilizers are stored at all: without them, finding J needs Gaussian elimination, which takes about n³ steps.

### 9.4 Worked example: measuring the Bell state

Tableau from Section 8.2: destabilizers +ZI, +IX; stabilizers +XX, +ZZ.

**Measure qubit 0.** Stabilizer +XX has an X on qubit 0, so the outcome is random (Case I), with p = the +XX row. No other row has an X on qubit 0, so step 1 does nothing. Row +XX moves to the destabilizer slot, and the stabilizer becomes ±ZI. Suppose the coin gives 0, so +ZI:

```
destabilizers: +XX  +IX      stabilizers: +ZI  +ZZ
```

**Measure qubit 1.** Neither +ZI nor +ZZ has an X on qubit 1, so the outcome is determined (Case II). Destabilizer 0 (+XX) and destabilizer 1 (+IX) both have an X on qubit 1, so multiply stabilizers 0 and 1: ZI × ZZ = +IZ. The sign is +, so the outcome is 0, the same as qubit 0, as the Bell state promises. ✓

### 9.5 In the code

`Tableau::measure(a, rng)` returns the outcome and whether it was random. Other entry points:

- `measure_forced(a, bit)` chooses the outcome instead of flipping a coin. It is useful for tests, and it refuses if the outcome is determined and different.
- `peek(a)` returns a determined outcome without changing anything.
- In Python, `t.measure(q)` returns 0 or 1. `t.measure(q, force=1)` is the forced version.

The random coin comes from a small built-in random number generator (`rng.rs`). Passing `seed=` makes runs repeatable.

> **Reading the code:** `Tableau::measure_with` in `tableau.rs`
>
> ```rust
> fn random_pivot(&self, a: usize) -> Option<usize> {
>     (self.n..2 * self.n).find(|&p| self.rows.x(p, a))   // first stabilizer with x_pa = 1
> }
>
> fn measure_with(&mut self, a: usize, choose: impl FnOnce() -> bool) -> Result<MeasureResult> {
>     self.check_qubit(a)?;                       // IndexError if a is out of range
>     let n = self.n;
>     match self.random_pivot(a) {
>         Some(p) => {                            // ---- Case I: random ----
>             for i in 0..2 * n {
>                 if i != p && i != p - n && self.rows.x(i, a) {
>                     self.rows.rowsum(i, p).map_err(|_| broken_invariants())?;
>                 }
>             }
>             self.rows.copy_row(p - n, p);       // step 2
>             self.rows.zero_row(p);              // step 3: row p = ±Z_a …
>             let outcome = choose();             // … with the sign from the coin
>             self.rows.set_r(p, outcome);
>             self.rows.set_z(p, a, true);
>             Ok(MeasureResult { outcome, deterministic: false })
>         }
>         None => Ok(MeasureResult {              // ---- Case II: determined ----
>             outcome: self.determinate_outcome(a)?,
>             deterministic: true,
>         }),
>     }
> }
>
> pub fn measure(&mut self, a: usize, rng: &mut Rng) -> Result<MeasureResult> {
>     self.measure_with(a, || rng.bit())          // the coin is a fair random bit
> }
> ```
>
> - `random_pivot` returns `Option<usize>`: `Some(p)` if a pivot row exists, otherwise `None`. The `match` turns the two cases of the paper into the two branches (Section 0.6).
> - `choose` is a closure (Section 0.8). `measure` passes `|| rng.bit()`, which flips a coin. `measure_forced` passes `|| outcome`, which always returns the requested bit. The algorithm is written once and the coin is swapped in.
> - `.map_err(|_| broken_invariants())?` means: if `rowsum` fails (which would only happen if a row was overwritten by hand), replace its error with a clearer message and return it (Section 0.7).
> - `MeasureResult { outcome, deterministic: false }` builds a small struct with two fields, the result and whether it was random.
>
> And Case II, exactly the derivation above:
>
> ```rust
> fn determinate_outcome(&mut self, a: usize) -> Result<bool> {
>     let n = self.n;
>     let scratch = 2 * n;                        // the spare row
>     self.rows.zero_row(scratch);                // start from +I
>     for i in 0..n {
>         if self.rows.x(i, a) {                  // destabilizer i has x = 1 on qubit a …
>             self.rows.rowsum(scratch, i + n)    // … so multiply in stabilizer i
>                 .map_err(|_| broken_invariants())?;
>         }
>     }
>     Ok(self.rows.r(scratch))                    // the sign is the outcome
> }
> ```

---

## 10. The CHP language and `.chp` files

The paper's Section IV describes **CHP** ("CNOT-Hadamard-Phase"), the authors' C program for this algorithm. Its input is a tiny language with four instructions, one per line:

```
c a b    CNOT, control a, target b
h a      Hadamard on qubit a
p a      Phase on qubit a
m a      measure qubit a and print the result
```

A **`.chp` file** is just a plain text file in that language; the extension is only a naming convention. Any text before a line that starts with `#` is a free description and is skipped. That rule (and the `.chp` name) come from the original CHP program, not from the paper's text. The number of qubits is the largest index used, plus one.

`examples/chp/` contains five programs:

| File | What it demonstrates | Expected result |
|---|---|---|
| `teleport.chp` | quantum teleportation, copied from the paper | the state on qubit 0 ends up on qubit 2 |
| `ghz.chp` | 3-qubit entanglement | three equal bits; the first random, the others determined |
| `dense_coding.chp` | sending 2 classical bits with 1 qubit | `1 1`, determined |
| `simon.chp` | Simon's algorithm with secret 110 | outcomes y always satisfy y₀ + y₁ even |
| `shor9.chp` | Shor's 9-qubit error-correcting code | detects and fixes a bit flip, then reads 0 |

CHP has no X (NOT) instruction, so the files build one from the others: X = H·P·P·H. P·P is Z, and H turns Z into X.

How to run them:

```
cargo run --release --bin chp -- examples/chp/ghz.chp        # command line
```
```python
from stabilizer import run_chp
outcomes, tableau = run_chp(open("examples/chp/ghz.chp").read(), seed=1)
# outcomes is a list of (qubit, result, was_it_determined)
```

`chp.rs` contains the parser. It reports mistakes with a line number, for example `line 3: unknown instruction "x"`.

> **Reading the code:** the heart of the parser in `chp.rs`
>
> ```rust
> let tokens: Vec<&str> = line.split_whitespace().collect();   // "c 0 1" → ["c", "0", "1"]
> if tokens.is_empty() {
>     continue;                                                 // skip blank lines
> }
> let (gate, arity) = match tokens[0].to_ascii_lowercase().as_str() {
>     "c" => (Gate::Cnot(arg(1)?, arg(2)?), 3),
>     "h" => (Gate::H(arg(1)?), 2),
>     "p" => (Gate::P(arg(1)?), 2),
>     "m" => (Gate::Measure(arg(1)?), 2),
>     other => return value_err(format!("line {lineno}: unknown instruction {other:?}")),
> };
> ```
>
> - `split_whitespace().collect()` is Python's `line.split()`. `Vec<&str>` is a list of borrowed pieces of the line.
> - `match` works on strings too. Each branch builds the matching `Gate` variant (Section 0.6) and records how many tokens the line must have (`arity`).
> - `arg(1)` is a small closure defined just above, which parses token 1 as a qubit number. Its `?` stops with a "line N: expected a qubit index" error if that fails.
> - `other =>` catches every unknown instruction. `{other:?}` in `format!` prints it with quotes, like Python's `repr`.

---

## 11. The timing experiment (Figure 2)

Gates always cost about n operations. A measurement's cost depends on **how many rows need a rowsum**, which is the number of rows with x = 1 on the measured qubit:

- in a fresh state almost no rows have x = 1, so measuring is cheap (about n);
- in a well-mixed, "dense" state about half the rows do, so measuring costs about n².

The paper asked when the switch happens. It generated random circuits with ⌊β·n·log₂n⌋ random gates (each CNOT, H or P with probability 1/3), then timed measuring every qubit. A small β gives sparse, cheap states; a large β gives dense, expensive ones. The transition is fairly sharp, which the paper calls a "phase transition".

The project reproduces it:

- `random_circuit(n, beta, seed)` builds one such circuit.
- `time_measurements(n, beta, seed)` returns seconds per measurement. Only the measurements are timed, inside Rust, so Python's overhead is not counted.
- `python examples/figure2.py` prints the whole table (add `--full` for the paper's sizes, `--plot file.png` for a chart).

On this machine at n = 1600, a measurement takes about 8 µs at β = 0.6 and about 180 µs at β = 1.2. Doubling the number of gates made measuring over 20 times slower, the same effect as in the paper. In absolute terms this is over 1000 times faster than the paper's 2004 computer.

---

## 12. Counting stabilizer states (Proposition 2)

How many different stabilizer states exist on n qubits? The paper proves

```
N(n) = 2ⁿ × (2ⁿ + 1)(2ⁿ⁻¹ + 1)…(2¹ + 1)
```

For one qubit that is 2 × 3 = **6**: |0⟩, |1⟩, |+⟩, |−⟩, and two more with stabilizers ±Y. For two qubits it is 60, and for three 1080. The number grows like 2^(n²/2), far fewer than all quantum states but still enormous.

> **Derivation: where the formula comes from**
>
> The idea is: count all ordered lists of generators, then divide by how many lists describe the same state.
>
> *One qubit, as a warm-up.* A state is fixed by one generator: any of the 3 non-identity Paulis (X, Y, Z) with either sign. That is 3 × 2 = 6 choices, and different choices give different states, so N(1) = 6.
>
> *Counting generator lists (ignoring signs first).* There are 4ⁿ Paulis on n qubits (4 letters per qubit).
>
> - The first generator M₁ can be anything except the identity: **4ⁿ − 1** choices.
> - The second must commute with M₁. Exactly half of all Paulis commute with a given non-identity Pauli, so 4ⁿ/2 commute with M₁. It must also not be in the group generated so far, {I, M₁}, which has 2 elements. That leaves **4ⁿ/2 − 2** choices.
> - The third must commute with M₁ and M₂ (one quarter of all Paulis: 4ⁿ/4) and lie outside the 4-element group they generate: **4ⁿ/4 − 4** choices.
> - In general, generator k + 1 has 4ⁿ/2ᵏ − 2ᵏ choices.
>
> Each generator can also take either sign, which gives another factor 2ⁿ.
>
> *Dividing out repeats.* A given stabilizer group has 2ⁿ elements (Theorem 1). The number of ordered generating lists for one fixed group is found the same way: (2ⁿ − 1)(2ⁿ − 2)(2ⁿ − 4)⋯, i.e. pick any non-identity element, then anything outside what is generated so far.
>
> Dividing the two counts and simplifying each factor gives 2ⁿ × ∏ₖ (2^{n−k} + 1). The paper shows the algebra in its proof of Proposition 2. For n = 1: 2 × (2 + 1) = 6 ✓.

> **Reading the code:** `num_stabilizer_states` in `counting.rs`
>
> ```rust
> pub fn num_stabilizer_states(n: u32) -> BigUint {
>     let one = BigUint::from(1u32);
>     let mut acc = &one << n;                    // 2^n
>     for k in 0..n {
>         acc *= (&one << (n - k)) + 1u32;        // times (2^(n−k) + 1)
>     }
>     acc
> }
> ```
>
> `BigUint` is an unbounded whole number from the `num-bigint` library, like Python's `int`, because the answer quickly exceeds 64 bits. `one << n` shifts 1 left by n places, which is 2ⁿ. The Python wrapper turns the result into an ordinary Python `int`.

`num_stabilizer_states(n)` computes it exactly (as a Python int, however large). The test checks the formula a second way. Starting from |00…0⟩, it applies every possible gate again and again, collects every distinct state it reaches, and stops when nothing new appears. It finds exactly 6, 60 and 1080 states.

To decide whether two tableaus are the *same state*, the code puts their stabilizers in a standard ("canonical") form, `canonical_stabilizers()`. Different-looking generator lists can describe the same state, for example {ZI, IZ} and {ZI, ZZ}, because ZZ = ZI × IZ.

---

## 13. Arithmetic with bits: GF(2)

Sections 14 to 19 use linear algebra on bits, so here is what that means.

**GF(2)** is arithmetic with only 0 and 1, where 1 + 1 = 0. Addition is XOR and multiplication is AND. A matrix of bits multiplies a vector of bits as usual, but with XOR instead of +.

**Gaussian elimination** is the standard school method for solving equations: repeatedly add one row to another to clear entries until the matrix is simple. Over GF(2), "add a row" means "XOR a row". `gf2.rs` provides it as `BitMatrix` with:

- `rank`: how many independent rows there are;
- `inverse`: the matrix that undoes this one;
- `solve`: find x with A·x = b.

**Symplectic completion** (`symplectic_completion`). Given some commuting stabilizers, it builds matching destabilizers, plus extra "logical" pairs if there are fewer than n stabilizers, so that the full Section 7.2 rules hold. It solves "find a Pauli that anticommutes with this stabilizer and commutes with those" as linear equations, then fixes up the results so they also commute with each other. This lets you build a tableau from any list of stabilizers: `Tableau.from_stabilizers(["+XX", "+ZZ"])`.

---

## 14. The canonical form (Section VI)

### 14.1 The question

Many different circuits do the same thing. For example, H followed by H does nothing at all. Two circuits are **equivalent** if they act the same on every input. The paper proves (**Theorem 8**) that every circuit made of H, P and CNOT is equivalent to one with a fixed layered shape, 11 layers in this order:

```
H  C  P  C  P  C  H  P  C  P  C
```

Each layer contains only one kind of gate (H = Hadamards, C = CNOTs, P = phases). This is useful for designing and shrinking circuits.

### 14.2 Tableaus identify circuits (Lemma 5)

Run a circuit (without measurements) on the starting tableau. The result records how the circuit transforms each X and Z, and that completely determines what the circuit does. So **two circuits are equivalent exactly when they produce the same tableau**. The tests use this to check every rewritten circuit: compare tableaus with `==`.

### 14.3 How the 11 layers are found

The proof is a recipe. Take any tableau and apply layers of gates that tidy it up, step by step, until it is the starting (identity) tableau. Split the tableau into four n×n blocks of bits:

```
( A | B )     A = x-bits of destabilizers,   B = z-bits of destabilizers
( C | D )     C = x-bits of stabilizers,     D = z-bits of stabilizers
```

Each gate layer changes these blocks in a predictable way:

- **Hadamards** swap columns between the x-blocks and z-blocks.
- **Phases** add x-columns into z-columns, so with the right choice they add 1s on the diagonal of a block.
- **CNOTs** add one column to another, the same row operation as Gaussian elimination but applied to columns.

> **Derivation: how each gate moves the blocks**
>
> Everything follows from the bit rules of Section 8, applied to **every** row at once. "Column a of the x-blocks" means the x bit of qubit a in every row.
>
> - **H on qubit a:** the rule "swap xᵢₐ and zᵢₐ" for every row i swaps column a of A with column a of B, and column a of C with column a of D.
> - **P on qubit a:** the rule "zᵢₐ ⊕= xᵢₐ" adds column a of A into column a of B, and column a of C into column a of D. If C = I (the identity), column a of C is a single 1 in row a, so P on qubit a adds 1 to entry (a, a) of D. That is the "add 1s on the diagonal" trick.
> - **CNOT from c to t:** the rule "x_t ⊕= x_c" adds x-column c into x-column t, and "z_c ⊕= z_t" adds z-column t into z-column c.
>
> In matrix language, a CNOT multiplies each x-block on the right by an elementary matrix E (column t += column c), and each z-block by Eᵀ (column c += column t). Over GF(2), E is its own inverse, so a whole layer of CNOTs that turns the x-blocks into X·L turns the z-blocks into Z·(L⁻¹)ᵀ. The paper writes this as "D(Mᵀ)⁻¹", and it explains step 4 of the table below: D = M·Mᵀ becomes M·Mᵀ·(Mᵀ)⁻¹ = M.

The 11 steps (in `to_identity_circuit`) aim for these shapes:

| Step | Layer | What it achieves |
|---|---|---|
| 1 | H | make C invertible (Lemma 6) |
| 2 | C | turn C into the identity matrix I |
| 3 | P | fix the diagonal of D so that D = M·Mᵀ for some M (Lemma 7) |
| 4 | C | make both C and D equal to M |
| 5 | P | clear D to zero, and fix the stabilizer signs |
| 6 | C | turn C back into I; B automatically becomes I |
| 7 | H | swap, giving (I A; 0 I) |
| 8 | P | fix the diagonal of A so that A = N·Nᵀ (Lemma 7 again) |
| 9 | C | make the top blocks N and N |
| 10 | P | clear the top-right block, and fix the destabilizer signs |
| 11 | C | finish: the identity tableau |

After each step the code checks that the blocks really have the promised shape.

> **Derivation: why B becomes I by itself in step 6**
>
> After step 6, the stabilizer blocks are C = I and D = 0. So stabilizer i is just **X on qubit i**. Proposition 3 says destabilizer j must anticommute with stabilizer i exactly when i = j. A Pauli anticommutes with Xᵢ exactly when it has a Z or Y on qubit i, which means its z bit at qubit i is 1. Destabilizer j's z bits are row j of B. So B_ji = 1 exactly when i = j, which says B = I. ∎
>
> The paper uses the same kind of argument ("commutativity implies…") for D being symmetric in step 3 and A being symmetric in step 8. With C = I, stabilizers i and j commute exactly when D_ij = D_ji, so D must be symmetric.

**Lemma 6** says which qubits to apply Hadamards to so that C has full rank. It comes down to Gaussian elimination on the stabilizer bits; see `lemma6_hadamards`.

**Lemma 7** says that for any symmetric bit matrix A (A equals its transpose), you can add 1s on some diagonal entries and get M·Mᵀ for an invertible M. Example: A = [[0,1],[1,1]]. Adding 1 to both diagonal entries gives [[1,1],[1,0]], which equals M·Mᵀ with M = [[1,0],[1,1]]. See `lemma7`.

> **Derivation: how Lemma 7 finds M**
>
> Choose M lower-triangular with 1s on the diagonal. Such a matrix is always invertible: going down the rows, each one introduces a new pivot. Entry (i, j) of M·Mᵀ is the sum over k of M_ik·M_jk. Since M is zero above its diagonal, only k ≤ i and k ≤ j contribute. For i > j (below the diagonal) that gives
>
> ```
> (M·Mᵀ)_ij = Σ_{k<j} M_ik·M_jk  +  M_ij·M_jj
>           = Σ_{k<j} M_ik·M_jk  +  M_ij            (because M_jj = 1)
> ```
>
> We want this to equal A_ij, so
>
> ```
> M_ij = A_ij + Σ_{k<j} M_ik·M_jk          (mod 2, where minus is the same as plus)
> ```
>
> Every M on the right has a smaller column index, so filling in M row by row, left to right, always uses values already computed. The diagonal of M·Mᵀ comes out to Σ_{k≤i} M_ik, which may not match A_ii, and that mismatch is exactly what the phase gates fix: Λ_ii = A_ii + Σ_{k≤i} M_ik. Matching above the diagonal is automatic because both A and M·Mᵀ are symmetric. ∎
>
> Example: A = [[0,1],[1,1]]. M₁₀ = A₁₀ = 1, so M = [[1,0],[1,1]] and M·Mᵀ = [[1,1],[1,0]]. Comparing diagonals: Λ₀₀ = 0 + 1 = 1 and Λ₁₁ = 1 + (1 + 1) = 1. So A + Λ = [[1,1],[1,0]] ✓.

**Signs (steps 5 and 10).** Two phases in a row (P·P = Z) flip the sign of every row with an X on that qubit and change nothing else. Choosing which qubits get the extra P·P is a small GF(2) equation, `M·v = signs`. The signs must be read *after* the "phase on every qubit" part of the layer, because that part also flips some signs (Section 23).

### 14.4 Which way round?

The recipe produces gates G that turn the circuit's tableau into the identity, i.e. G *undoes* the circuit. The paper notes this (footnote 46). So `canonical_form(circuit)` runs the recipe on the *inverse* circuit, which gives gates equal to the original circuit, in H-C-P-C-P-C-H-P-C-P-C order. Related functions:

- `canonical_rounds(circuit)`: the same result, split into its 11 layers;
- `synthesize(tableau)`: a circuit that produces a given tableau;
- `to_identity_circuit(tableau)`: the undoing circuit itself.

### 14.5 Fewer CNOTs (Corollary 9, `pmh.rs`)

H and P layers have at most a few gates per qubit. CNOT layers can be large: plain Gaussian elimination uses about n² CNOTs. The **Patel–Markov–Hayes (PMH)** algorithm needs only about n²/log n. It works on groups of columns and first removes rows that repeat the same bit pattern within the group, which handles many rows at once. Use it with `canonical_form(circuit, method="pmh")`, or directly with `cnot_synthesis(matrix)`, which returns a CNOT list that computes |v⟩ → |A·v⟩. On a random 256×256 matrix it uses clearly fewer gates than plain elimination, and a test checks this.

---

## 15. Inner product of two states

The **inner product** |⟨ψ|φ⟩| measures how similar two states are: 1 means identical, 0 means completely distinguishable. For stabilizer states it can only be 0 or 1/√2 raised to some power, i.e. 2^(−s/2) for a whole number s.

How the code finds it (`inner_product.rs`), following the paper:

1. Use the canonical-form recipe to find gates U that turn ψ into |00…0⟩.
2. Apply the same gates to φ. Applying the same gates to both states doesn't change their similarity.
3. Now compare φ with |00…0⟩. Let k be the number of stabilizers of φ that contain X or Y after Gaussian elimination. If one of the remaining Z-only stabilizers has a − sign, φ and |00…0⟩ disagree for certain, so the answer is 0. Otherwise the answer is 2^(−k/2).

Example from the paper: the Bell state (+XX, +ZZ) and |00⟩ (+ZI, +IZ) give 1/√2. Indeed (|00⟩+|11⟩)/√2 contains |00⟩ with weight 1/√2.

> **Derivation: why the answer is 2^(−k/2) or 0**
>
> *Why step 2 is allowed.* For any unitary U, ⟨Uψ|Uφ⟩ = ⟨ψ|U†U|φ⟩ = ⟨ψ|φ⟩, because U†U = I. So we may replace (ψ, φ) by (|00…0⟩, Uφ).
>
> *What φ looks like after elimination.* Gaussian elimination on the x bits splits φ's generators into two groups:
>
> - k "flip" generators, each with some X or Y, and
> - n − k "Z-only" generators, made of I's and Z's only.
>
> Look at the Z-only ones first. A Z-only Pauli is diagonal: on a basis state |b⟩ (b a bit string) it gives ±1. "φ is stabilized by it" means φ can only contain basis states |b⟩ where it gives +1. Each Z-only generator is one linear (XOR) condition on b, and there are n − k independent ones. So the allowed b's form a set of 2ⁿ / 2^{n−k} = **2^k** bit strings.
>
> The k flip generators each flip some bits and add phases, mapping allowed basis states to other allowed ones. Being stabilized by them means φ's amplitudes on those states are related by phases only, so all 2^k allowed basis states have the **same magnitude**. To be normalized, that magnitude is 1/√(2^k) = 2^{−k/2}.
>
> *The overlap with |00…0⟩.* ⟨00…0|φ⟩ is just φ's amplitude on the all-zeros string.
>
> - Every Z-only Pauli with sign + gives +1 on |00…0⟩. So if all Z-only generators have sign +, all-zeros is allowed and the overlap has size 2^{−k/2}.
> - If some Z-only generator has sign −, it gives −1 on |00…0⟩, so all-zeros is forbidden and the overlap is 0. ∎
>
> Bell example: generators +XX (flip) and +ZZ (Z-only). k = 1, and +ZZ has sign +, so the answer is 2^{−1/2} = 1/√2 ✓.

In Python: `inner_product(a, b)` returns the number, and `inner_product_exponent(a, b)` returns s (or `None` if the answer is 0).

---

## 16. Section V: how hard is this problem?

### 16.1 The claim, in plain words

Computer scientists sort problems into **complexity classes** by the resources needed to solve them. The paper shows that simulating stabilizer circuits is exactly as hard as simulating circuits made only of **CNOT** (and NOT) gates. In jargon, it is "complete for ⊕L" ("parity-L"). The consequence: adding H and P to CNOT gives no real extra power. That helps explain why these circuits are easy to simulate. They are probably not even powerful enough for all ordinary computation.

### 16.2 The idea of the proof

The x and z bits of the tableau change in a simple way: each update is an XOR of other bits. A CNOT-only circuit can compute exactly that. The difficulty is the sign bits, which change in a more complicated way.

The proof shows that a measurement's outcome can still be found while tracking only **one number α**, which is always 1, i, −1 or −i (so it fits in 2 bits). It never stores the sign bits. α is built in two passes:

1. multiply together the stabilizer rows that a Case II measurement would combine, column by column, keeping only the factor of i (as if no gate had ever changed a sign);
2. go back through every gate and flip α's sign each time that gate would have flipped one of those rows' signs.

At the end α = +1 means outcome 0 and α = −1 means outcome 1.

> **Derivation: why splitting α into two passes is allowed**
>
> In Case II the outcome is the sign of the product ∏_{i∈J} Sᵢ (Section 9.3). Write each stabilizer as Sᵢ = (−1)^{rᵢ}·Lᵢ, where Lᵢ is its letter part. Signs are plain numbers, so they can be pulled to the front of the product:
>
> ```
> ∏_{i∈J} Sᵢ = (∏_{i∈J} (−1)^{rᵢ}) · (∏_{i∈J} Lᵢ)
> ```
>
> - The second factor depends only on x and z bits. Multiplied out column by column, it gives a phase of the form i^{something}. That is **pass 1**.
> - Each rᵢ starts at 0 and is changed only by gates, and each change is "flip it or not" (XOR, Section 8). So (−1)^{rᵢ} is −1 raised to the number of flips, and **pass 2** counts the flips one gate at a time instead of storing rᵢ.
>
> Multiplying the two passes gives the full sign, and nothing in either pass ever needs a stored sign bit. ∎

### 16.3 In the code

`determinate_outcome(circuit, qubit)` (in `parity_l.rs`) runs that procedure. It asks an "oracle" (a helper that answers "what was bit x or z of row i after t gates?") instead of looking at a tableau. Our oracle simply replays the circuit and remembers every step. The test checks the answer against the normal tableau for hundreds of random circuits. It returns `None` when the outcome is random. As in the paper, the circuit may not contain measurements; they are assumed to happen at the end.

---

## 17. Mixed states (Section VII A)

### 17.1 What a mixed state is

So far a state was known exactly ("pure"). A **mixed state** includes classical uncertainty: "the qubit is |0⟩ or |1⟩, we don't know which, 50/50". The extreme case is the **maximally mixed** qubit, a fair coin with no quantum structure at all.

Mixed states are described by a **density matrix** ρ, a 2ⁿ×2ⁿ matrix. For the stabilizer mixed states of the paper, ρ is determined by r < n stabilizers:

```
ρ = (1/2ⁿ) × (I + M₁)(I + M₂)…(I + M_r)
```

Fewer stabilizers means more uncertainty. r = n is a pure state, and r = 0 is completely random.

> **Derivation: checking the formula on one qubit**
>
> - r = 0 (no stabilizers): ρ = (1/2)·I, the maximally mixed qubit. Measuring gives 0 or 1 with probability ½ each ✓.
> - r = 1 with M₁ = +Z: ρ = (1/2)(I + Z) = (1/2)·diag(2, 0) = diag(1, 0) = |0⟩⟨0| ✓.
> - r = 1 with M₁ = −Z: ρ = (1/2)(I − Z) = |1⟩⟨1| ✓.
>
> *Why it works in general.* (I + M)/2 is the projector onto M's +1 eigenspace: (I + M)/2 · (I + M)/2 = (I + 2M + M²)/4 = (2I + 2M)/4 = (I + M)/2, because M² = I. Squaring it changes nothing, which is what makes it a projector. Commuting projectors multiply to the projector onto their common +1 space. Each independent generator halves the space, leaving 2^{n−r} dimensions. ρ is "equally likely to be anywhere in that space": the projector divided by its dimension, 2^{−(n−r)} ∏(I + Mᵢ)/2 = 2^{−n} ∏(I + Mᵢ).

### 17.2 The mixed tableau

`MixedTableau` keeps the r stabilizers and their destabilizers. To keep the neat structure of Section 7.2, it also keeps n − r pairs of **logical operators**, written X̄ and Z̄. These are bookkeeping rows describing the "unknown" part. Layout:

```
rows 0 .. r        destabilizers
rows r .. n        X̄ (logical) rows
rows n .. n+r      stabilizers
rows n+r .. 2n     Z̄ (logical) rows
```

Row i and row i ± n are partners: they anticommute, and every other pair commutes. Gates work exactly as before. `MixedTableau(n, r)` starts with qubits 0..r in |0⟩ and the rest maximally mixed.

### 17.3 Measuring: three cases

- **Case I:** a stabilizer anticommutes with Zₐ. This is random, exactly as before.
- **Case II:** Zₐ is (± a product of) stabilizers. This is determined, exactly as before.
- **Case III:** Zₐ commutes with every stabilizer but is not one of them. Measuring "an unknown qubit" gives a fair coin, and afterwards we *know* the answer, so the state gains a stabilizer and r grows by 1. The code reuses the Case I steps with a logical row in the role of row p, then moves the new stabilizer into place.

Example: a maximally mixed qubit (r = 0). The first measurement is random and r becomes 1. Measuring again gives the same result, now determined.

> **Derivation: why Case III is exactly 50/50**
>
> The probability of outcome 0 is Tr[ρ (I + Zₐ)/2] = ½ + ½ Tr[Zₐ ρ]. Expand ρ = 2^{−n} ∏(I + Mᵢ) into a sum of products of generators, then multiply each term by Zₐ. Every resulting term is ± a Pauli of the form Zₐ·(product of some Mᵢ).
>
> - A non-identity Pauli has trace 0, because its diagonal entries cancel in pairs (try Z: 1 + (−1) = 0).
> - In Case III, Zₐ is *not* ± a product of the Mᵢ, so Zₐ·(product) is never ± the identity.
>
> So every term has trace 0, Tr[Zₐ ρ] = 0, and p(0) = ½. ∎
>
> In Case II, by contrast, one of those products *equals* ±Zₐ, so one term is ±I, whose trace is 2ⁿ. That is the determined outcome.

### 17.4 Throwing a qubit away

`discard(a)` removes qubit a (a "partial trace"), leaving n − 1 qubits. The stabilizers that don't involve qubit a survive; the others are combined so that at most two involve qubit a, and those are dropped. The logical rows are then rebuilt with symplectic completion (Section 13). Example: discarding one half of a Bell pair leaves a maximally mixed qubit, because that half alone looks completely random.

---

## 18. Unusual starting states (Section VII B)

### 18.1 The situation

The circuit still uses only H, P and CNOT, but the starting state may be *any* state, as long as it is a product of small independent pieces ("blocks") of at most a few qubits each. Some useful quantum algorithms need such inputs, for example "magic states". The price: only a small number d of measurements is allowed, because the work grows like 4^d.

### 18.2 The trick: move the measurement back to the start

Instead of pushing the state forward through the gates, push each measurement *backwards* to the beginning. "Measure Zₐ after gates U" is the same as "measure U†ZₐU at the start", and because U is made of our three gates, U†ZₐU is again just a Pauli (Section 4.3). So every measurement becomes a Pauli question about the *initial* product state.

The probability of a sequence of outcomes is a formula involving these Paulis and the initial state. Expanding it gives 2^(2d−1) simple pieces. Each piece is the "average value" of one Pauli in the initial state, and for a product state that is just a product of one small number per block.

> **Derivation: one measurement**
>
> After the gates U, the state is UρU†. Outcome 0 of measuring qubit a has projector Π = (I + Zₐ)/2, so
>
> ```
> p(0) = Tr[ Π · UρU† ]
>      = Tr[ U†ΠU · ρ ]                  (the trace is cyclic: Tr[AB] = Tr[BA])
>      = Tr[ (I + U†ZₐU)/2 · ρ ]         (U†IU = I)
>      = ½ + ½·Tr[Q ρ]                   with Q = U†ZₐU, and Tr[ρ] = 1
> ```
>
> Q is a Pauli, because U is built from H, P and CNOT (Section 4.3), so it splits into one piece per block: Q = Q₁ ⊗ Q₂ ⊗ …. The state also splits, ρ = ρ₁ ⊗ ρ₂ ⊗ …, and the trace of a tensor product is the product of the traces:
>
> ```
> Tr[Q ρ] = Tr[Q₁ρ₁] × Tr[Q₂ρ₂] × …
> ```
>
> Each factor involves only one small block matrix, so it is cheap.
>
> **Two measurements.** With pulled-back projectors Π₁ = (I ± Q₁)/2 and Π₂ = (I ± Q₂)/2:
>
> ```
> p(m₁, m₂) = Tr[ Π₂ Π₁ ρ Π₁ Π₂ ] = Tr[ Π₂ Π₁ ρ Π₁ ]
> ```
>
> The second form moves the last Π₂ to the front (cyclic trace) and uses Π₂Π₂ = Π₂, as for any projector. Each Π is a sum of 2 terms, so the three remaining Π's give 2 × 2 × 2 = 8 terms, each a Pauli product traced against ρ. In general d measurements give 2^d × 2^(d−1) = 2^(2d−1) terms, the paper's count. That growth is why d must stay small.
>
> **Computing one block trace.** A Pauli matrix has exactly one nonzero entry per row. P|v⟩ = (phase)·|v ⊕ xmask⟩, where xmask marks the qubits with X or Y. So Tr[Pρ] = Σ_v phase(v)·ρ[v, v ⊕ xmask], one pass down the block's rows. Here phase(v) = i^{#Y}·(−1)^{(number of 1s of v where P has Z or Y)}.

### 18.3 In the code

```python
import numpy as np
from stabilizer import ProductStateSimulator

psi = np.array([1, np.exp(1j*np.pi/4)]) / np.sqrt(2)      # T|+> "magic state"
sim = ProductStateSimulator(1, [([0], np.outer(psi, psi.conj()))])
sim.apply_h(0)
sim.probability(0)        # ≈ 0.854 = (1 + cos 45°)/2
sim.measure(0)            # samples 0 or 1 with that probability
```

Each block is given as (list of qubits, density matrix). Qubits not mentioned start as |0⟩. Inside a block's matrix, the first listed qubit is the most significant bit, the same order as `numpy.kron`.

---

## 19. A few non-stabilizer gates (Section VII C)

### 19.1 The situation

Now the start is |00…0⟩, but a few gates outside H/P/CNOT are allowed. An example is the **T gate**, diag(1, e^{iπ/4}), "half a phase gate". With enough T gates you get a fully general quantum computer, so this cannot be cheap in general. The cost grows exponentially with the number of such gates, but only polynomially with the number of qubits.

### 19.2 The idea

Write the state as a **sum of terms**. Each term is a stabilizer-like object: a number c, a Pauli P, and a list of sign flips s for the stabilizers. Start with a single term.

- **H, P, CNOT** update each term with the usual Pauli rules, so the term count stays the same.
- **A non-stabilizer gate U** is first written as a sum of Paulis (any small matrix can be). Applying it splits each term into several new terms. That is where the exponential cost comes from. Terms that become identical are merged.
- **Measuring a Pauli Q** keeps only the parts of each term consistent with the outcome. The paper's two cases (Q commutes with all stabilizers, or with all but one) say exactly how. The term count never grows here.
- **Probabilities:** each term contributes a simple number, and the sum is the probability.

> **Derivation: the term representation**
>
> Write ρ_s for the stabilizer state with generators M₁…Mₙ, where generator j has its sign flipped when sⱼ = 1:
>
> ```
> ρ_s = 2^{−n} (I + ε₁M₁)(I + ε₂M₂)⋯(I + εₙMₙ),     εⱼ = (−1)^{sⱼ}
> ```
>
> *Fact 1: a Pauli just flips signs as it moves through ρ_s.* Take one factor and a Pauli P. If P commutes with Mⱼ:
>
> ```
> (I + εⱼMⱼ) P = P + εⱼ MⱼP = P + εⱼ PMⱼ = P (I + εⱼMⱼ)
> ```
>
> If P anticommutes with Mⱼ, then MⱼP = −PMⱼ, and the same steps give P(I − εⱼMⱼ): the sign of generator j flips. Doing this for every factor:
>
> ```
> ρ_s · P = P · ρ_{s ⊕ σ(P)}
> ```
>
> where σ(P) has a 1 in position j exactly when P anticommutes with Mⱼ.
>
> *Fact 2: writing a gate as a sum of Paulis.* The 4^b Paulis on b qubits form a basis of all 2^b × 2^b matrices, and they are orthogonal: Tr[PᵢPⱼ] is 2^b when i = j and 0 otherwise. (PᵢPⱼ is ± a non-identity Pauli unless i = j, and non-identity Paulis have trace 0.) So if U = Σⱼ uⱼPⱼ, multiplying both sides by Pᵢ and taking the trace leaves one term:
>
> ```
> Tr[Pᵢ U] = Σⱼ uⱼ Tr[Pᵢ Pⱼ] = uᵢ · 2^b     →     uᵢ = Tr[Pᵢ U] / 2^b
> ```
>
> *Putting them together.* Applying U to one term c·P·ρ_s:
>
> ```
> U (c P ρ_s) U† = Σᵢ Σₖ uᵢ ūₖ · c · Pᵢ P ρ_s Pₖ          (U† = Σₖ ūₖ Pₖ, since Paulis are Hermitian)
>                = Σᵢ Σₖ uᵢ ūₖ c · (Pᵢ P Pₖ) · ρ_{s ⊕ σ(Pₖ)}     (Fact 1, moving Pₖ to the left)
> ```
>
> Every new term has the same shape: a number, a Pauli, and a sign pattern. That is why this representation is closed under gates, and why each gate multiplies the number of terms by at most (4^b)² = 4^{2b}.
>
> *Trace of a term.* Tr[P ρ_s] = ⟨ψ_s|P|ψ_s⟩, the expectation of P in the pure state ψ_s. If P anticommutes with some Mⱼ, it is 0 (the same argument as the Case I derivation in Section 9.2). Otherwise P = φ·∏_{j∈J} Mⱼ for some phase φ (Section 9.3), and the expectation is φ·∏_{j∈J} εⱼ.

### 19.3 In the code

```python
from stabilizer import NonStabilizerSimulator
sim = NonStabilizerSimulator(1)
sim.apply_h(0); sim.apply_t(0); sim.apply_h(0)
sim.probability(0)         # ≈ 0.854, the same as Section 18 by a different route
sim.num_terms              # how many terms the sum currently has
```

Other methods:

- `apply_unitary(matrix, qubits)`: any small gate, given as a numpy matrix;
- `measure_pauli("+XZ")`: measure any multi-qubit Pauli;
- `probability_pauli(...)`: the probability of +1 for such a measurement, without measuring.

---

## 20. How the code is organised

### 20.1 Rust and Python together

Rust does all the work; Python is a thin layer on top. The tools that connect them:

- **pyo3** is a Rust library that lets Rust code define Python classes and functions. All of that "glue" lives in `src/python.rs`.
- **maturin** compiles the Rust code into a Python extension file (`_core.pyd` on Windows) and installs it, so `import stabilizer` works.
- `Cargo.toml` describes the Rust project. The glue is compiled only when the `python` feature is on, so `cargo test` works without Python.
- `pyproject.toml` tells maturin where to put things.
- `python/stabilizer/__init__.py` re-exports everything from `_core`.
- `_core.pyi` lists every function and its types, so editors can autocomplete.

> **Reading the code:** how a Rust type becomes a Python class (`python.rs`)
>
> ```rust
> #[pyclass(name = "Tableau", module = "stabilizer._core", skip_from_py_object)]
> #[derive(Clone)]
> struct PyTableau {
>     inner: Tableau,      // the real Rust tableau
>     rng: Rng,            // the coin used for random outcomes
> }
>
> #[pymethods]
> impl PyTableau {
>     #[new]
>     #[pyo3(signature = (n, seed=None))]
>     fn new(n: isize, seed: Option<u64>) -> PyResult<Self> {
>         if n < 1 {
>             return Err(PyValueError::new_err(format!("n must be >= 1, got {n}")));
>         }
>         Ok(PyTableau { inner: Tableau::new(n as usize), rng: Rng::from_seed(seed) })
>     }
>
>     fn apply_h(&mut self, qubit: usize) -> PyResult<()> {
>         self.inner.h(qubit).py()
>     }
>
>     #[pyo3(signature = (qubit, force=None))]
>     fn measure(&mut self, qubit: usize, force: Option<i64>) -> PyResult<u8> {
>         let r = match force_arg(force)? {
>             Some(b) => self.inner.measure_forced(qubit, b),
>             None => self.inner.measure(qubit, &mut self.rng),
>         };
>         Ok(r.py()?.outcome as u8)
>     }
> }
> ```
>
> - `#[pyclass(name = "Tableau", ...)]` tells pyo3 to make this struct a Python class called `Tableau`. The Rust name `PyTableau` is only used inside Rust.
> - `#[pymethods]` turns every function in the `impl` block into a Python method. `#[new]` marks the constructor, i.e. Python's `__init__`.
> - `#[pyo3(signature = (n, seed=None))]` gives Python default arguments, so `Tableau(3)` and `Tableau(3, seed=7)` both work. `Option<u64>` receives `None` or an int.
> - `PyResult<T>` is a `Result` whose error is a Python exception. Returning `Err(PyValueError::new_err(...))` makes Python raise `ValueError`.
> - `.py()` is a small helper defined at the top of `python.rs`. It converts the project's `Error::Index` and `Error::Value` into Python's `IndexError` and `ValueError`.
> - Python types are converted automatically at the boundary: a Python `int` becomes `usize`, `None` becomes `Option::None`, a Rust `String` becomes a Python `str`, and a `Vec<String>` becomes a `list[str]`.
>
> Plain functions use `#[pyfunction]`. The final `#[pymodule] fn _core(...)` block registers every class and function into the `stabilizer._core` module that Python imports.

### 20.2 Files

| File | Contents |
|---|---|
| `src/lib.rs` | list of modules |
| `src/error.rs` | the error type: `Index` (becomes Python `IndexError`) or `Value` (becomes `ValueError`) |
| `src/rng.rs` | small random number generator with seeds |
| `src/pauli.rs` | `g`, `PauliString`, multiplication, gate rules for one Pauli |
| `src/rows.rs` | packed tableau rows, `rowsum`, the three gate updates |
| `src/tableau.rs` | `Tableau`: start state, gates, measurement, checks |
| `src/circuit.rs` | `Gate`, `Circuit` |
| `src/chp.rs`, `src/bin/chp.rs` | CHP parser and command-line program |
| `src/experiment.rs` | the Figure 2 experiment |
| `src/counting.rs` | Proposition 2 |
| `src/gf2.rs` | bit matrices, elimination, symplectic completion |
| `src/canonical.rs`, `src/pmh.rs` | Section VI |
| `src/inner_product.rs` | inner product |
| `src/parity_l.rs` | Section V |
| `src/mixed.rs` | Section VII A |
| `src/nonstab_init.rs` | Section VII B |
| `src/nonstab_gates.rs` | Section VII C |
| `src/python.rs` | Python bindings |
| `tests/*.py` | Python tests; `tests/reference.py` is a slow, simple numpy simulator used as a referee |
| `examples/` | `.chp` programs and `figure2.py` |

### 20.3 Building and testing

```
.venv\Scripts\maturin develop --release   # compile Rust and install into the virtual environment
.venv\Scripts\python -m pytest            # run the Python tests
cargo test --release                      # run the Rust tests
```

Rerun the maturin line after any change to Rust code.

---

## 21. Using it from Python

```python
from stabilizer import Tableau, Circuit

t = Tableau(2, seed=7)       # 2 qubits in |00>, seeded coin flips
t.apply_h(0)
t.apply_cnot(0, 1)           # Bell state
print(t.stabilizers())       # ['+XX', '+ZZ']
print(t)                     # whole tableau, destabilizers first
t.is_deterministic(0)        # False: the outcome will be random
a = t.measure(0)             # 0 or 1
b = t.measure(1)             # always equal to a
t.measure(0, force=1)        # choose the outcome (error if impossible)
```

Building and analysing circuits:

```python
from stabilizer import Circuit, canonical_form, determinate_outcome

c = Circuit(3)
c.h(0); c.cnot(0, 1); c.p(2)
c.gates()                    # [('h', 0), ('c', 0, 1), ('p', 2)]
c.tableau() == canonical_form(c).tableau()   # True: same circuit, 11-layer form
determinate_outcome(c, 2)    # 0 (qubit 2 is still |0>, up to phase)
```

Paulis are always strings like `"+XIZ"` (qubit 0 first; a missing sign means +). Small helpers: `pauli_product("+X", "+Z")` gives `'-iY'`, and `pauli_commutes("+XX", "+ZZ")` gives `True`.

Low-level access (used by the tests): `get_x(row, qubit)`, `set_z(...)`, `get_r(row)`, `row_to_pauli(row)`, `set_row_from_pauli(row, "+XZ")`, `rowsum(h, i)` and `to_numpy()`. Setting rows by hand can break the Section 7.2 rules; `check_invariants()` tells you if it did.

---

## 22. How we know it is correct (the tests)

The main strategy is a **referee**: `tests/reference.py` is a short, slow numpy simulator that stores all 2ⁿ numbers and multiplies actual matrices. It is too slow for big circuits but easy to trust. The tests run random circuits on both and compare results after every step.

| Test file | What it checks |
|---|---|
| `test_g.py` | all 16 cases of g, against a hand-written table and against real 2×2 matrices |
| `test_tableau.py` | starting tableau, getting/setting bits, printing, rowsum against a simple reference, including sizes 63/64/65 qubits where packing bugs would show |
| `test_gates_measure.py` | random circuits with measurements: every stabilizer really leaves the referee's state unchanged, and random/determined outcomes match the referee's probabilities |
| `test_chp.py` | the parser, the five example programs, the command-line tool |
| `test_canonical.py` | canonical form equals the original circuit, has the 11 layers in order; Lemmas 6 and 7; CNOT synthesis |
| `test_inner_product.py` | paper examples and random pairs against the referee |
| `test_parity_l.py` | Section V's procedure against the tableau |
| `test_counting.py` | 6, 60, 1080 by brute-force search |
| `test_mixed.py` | mixed tableau against the referee's density matrices, including discard |
| `test_nonstab_init.py` | Section VII B probabilities against the referee |
| `test_nonstab_gates.py` | Section VII C probabilities with T gates and random gates against the referee |
| `test_experiment.py` | random circuit shape; dense states measure slower than sparse ones |

The Rust side has 42 more unit tests inside the modules (`cargo test`).

---

## 23. Where we differ from the paper

None of these change any results; each is either a fix or a gap the paper leaves open.

1. **Measurement Case I** skips one multiplication, rowsum(p−n, p). It would produce an invalid ±i factor and is overwritten immediately anyway (Section 9.2).
2. **Mixed-state starting state.** The paper's text says the first n − r qubits are |0⟩, but its own formulas say the first r. We follow the formulas, since r stabilizers fix r qubits.
3. **Mixed-state Case III.** The paper's two row swaps cancel each other in one edge case, when the chosen row is the first logical row. The code first swaps a logical pair's roles, so the swaps always do what is intended.
4. **Discarding a qubit** takes about n³ steps instead of the paper's n². The paper skips the details as "rather involved".
5. **Canonical form, steps 5 and 10.** The signs must be read after the "phase on every qubit" gates, since those gates also flip signs. An early version read them before, the built-in check caught it, and it is fixed.
6. **Section VII B** uses a faster formula for each small trace (2^b instead of 4^b steps per block), and allows at most 12 measurements.

---

## 24. Glossary

| Term | Meaning |
|---|---|
| **qubit** | quantum bit; its state is a combination α\|0⟩ + β\|1⟩ |
| **gate** | an operation on qubits; mathematically a matrix |
| **H, P (S), CNOT** | the three stabilizer gates (Section 2.2) |
| **Pauli** | one of I, X, Y, Z, or a tensor product of them with a sign |
| **commute / anticommute** | AB = BA / AB = −BA |
| **stabilizer** | a Pauli that leaves the state unchanged |
| **generators** | n independent commuting stabilizers that pin down the state |
| **destabilizer** | helper row that makes measurement fast (Section 7.2) |
| **tableau** | the table of destabilizers and stabilizers, stored as bits |
| **rowsum** | multiply one tableau row into another, keeping the sign right |
| **phase / sign** | the factor +1, −1, +i or −i in front of a Pauli |
| **XOR (⊕)** | bitwise "exactly one is 1"; addition in GF(2) |
| **popcount** | number of 1 bits in a word |
| **GF(2)** | arithmetic on bits where 1 + 1 = 0 |
| **symplectic product** | the bit test for whether two Paulis anticommute |
| **density matrix** | description of a state including classical uncertainty |
| **mixed / pure state** | with / without classical uncertainty |
| **equivalent circuits** | circuits that act identically on every input |
| **canonical form** | the standard 11-layer shape of Theorem 8 |
| **inner product** | how similar two states are, from 0 to 1 |
| **CHP** | the paper's simulator program, and its 4-instruction language |
| **pyo3 / maturin** | the tools that expose Rust code to Python |

Rust terms (Section 0):

| Term | Meaning |
|---|---|
| **crate** | a Rust project or library; `crate::` means "this project" |
| **module** | one `.rs` file; declared with `mod`, imported with `use` |
| **`let` / `let mut`** | declare an unchangeable / changeable variable |
| **`usize`, `u64`, `i64`, `u8`** | whole-number types: size/index, 64-bit unsigned, 64-bit signed, 0..255 |
| **`Vec<T>`** | a growable list (Python `list`) |
| **slice `&[T]`** | a borrowed, read-only view of part of a list |
| **borrow (`&`, `&mut`)** | lend data without copying: read-only or writable |
| **struct / `impl`** | a record of fields / the block that gives it methods |
| **enum / `match`** | a type with several variants / a statement that handles each variant |
| **`Option<T>`** | `Some(value)` or `None` |
| **`Result<T>`, `?`** | `Ok(value)` or `Err(error)`; `?` passes an error up to the caller |
| **closure** | an inline function, `\|x\| x + 1`, like Python's `lambda` |
| **trait** | a set of methods a type promises to have, like an interface |
| **macro** (`name!`) | a code generator: `println!`, `format!`, `vec!`, `assert!` |
| **attribute** (`#[...]`) | an instruction to the compiler: `#[derive]`, `#[test]`, `#[pyclass]` |
| **panic** | a crash for a situation that should be impossible (`unwrap`, `expect`, `assert!`) |
