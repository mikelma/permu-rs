//! # permu-rs
//!
//! `permu-rs` is a collection of utilities for permutations. It contains useful 
//!

pub mod permutation;

/// Trait for learnable populations.
pub trait Learnable {
    /// Learns a probability distribution from a population of solutions.
    fn learn(&self) -> dyn Sampleable;
}

/// Trait for different sampleable distribution types.
pub trait Sampleable {
    /// Samples a solution from a sampleable distribution.
    fn sample(&self, pop: &mut dyn Learnable) -> Result<(), &'static str>;
}
