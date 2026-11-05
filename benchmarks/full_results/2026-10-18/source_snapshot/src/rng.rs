//! Small, dependency-free, seedable random number generator (xoshiro256**).
//!
//! Measurements with random outcomes need fair coin flips; seeding makes
//! every simulation reproducible.

use std::hash::{BuildHasher, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// Deterministic generator; the state is filled with SplitMix64.
    pub fn seed_from_u64(seed: u64) -> Self {
        let mut sm = seed;
        let mut next = || {
            sm = sm.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = sm;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        Rng {
            s: [next(), next(), next(), next()],
        }
    }

    /// Seeded from the clock and the process's random hash keys.
    pub fn from_entropy() -> Self {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        h.write_u128(nanos);
        Self::seed_from_u64(h.finish())
    }

    pub fn from_seed(seed: Option<u64>) -> Self {
        match seed {
            Some(s) => Self::seed_from_u64(s),
            None => Self::from_entropy(),
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// A fair coin.
    pub fn bit(&mut self) -> bool {
        self.next_u64() >> 63 == 1
    }

    /// Uniform in [0, 1).
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Uniform in [0, n). `n` must be positive.
    pub fn below(&mut self, n: usize) -> usize {
        ((self.next_u64() as u128 * n as u128) >> 64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_is_reproducible() {
        let mut a = Rng::seed_from_u64(7);
        let mut b = Rng::seed_from_u64(7);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn coin_is_roughly_fair() {
        let mut r = Rng::seed_from_u64(1);
        let ones = (0..10_000).filter(|_| r.bit()).count();
        assert!((4_700..5_300).contains(&ones), "{ones}");
    }

    #[test]
    fn below_in_range() {
        let mut r = Rng::seed_from_u64(3);
        for _ in 0..1000 {
            assert!(r.below(7) < 7);
        }
    }
}
