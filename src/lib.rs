//! # permu-rs
//!
//! `permu-rs` is a collection of utilities for permutations. It contains useful 
//!
pub mod permutation;

/// Trait for learnable populations.
pub trait Population {
    /// Learns a probability distribution from a `Population` and samples another `Population`.
    fn sample(&self, out: &mut Self) -> Result<(), &'static str>;
}
