//! Proposition 2: the number of pure n-qubit stabilizer states is
//! N = 2^n ∏_{k=0}^{n−1} (2^{n−k} + 1) = 2^{(1/2 + o(1)) n²}.

use num_bigint::BigUint;

pub fn num_stabilizer_states(n: u32) -> BigUint {
    let one = BigUint::from(1u32);
    let mut acc = &one << n;
    for k in 0..n {
        acc *= (&one << (n - k)) + 1u32;
    }
    acc
}

/// log₂ N, computed without big integers.
pub fn log2_num_stabilizer_states(n: u32) -> f64 {
    n as f64
        + (0..n)
            .map(|k| ((2f64).powi((n - k) as i32) + 1.0).log2())
            .sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_values() {
        let got: Vec<String> = (1..=4)
            .map(|n| num_stabilizer_states(n).to_string())
            .collect();
        assert_eq!(got, ["6", "60", "1080", "36720"]);
        assert!((log2_num_stabilizer_states(3) - 1080f64.log2()).abs() < 1e-9);
    }
}
