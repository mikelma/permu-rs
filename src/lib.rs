//! # permu-rs
//!
//! `permu-rs` is a collection of utilities for permutations. It contains useful 
//!

pub mod permutation;

/// Trait for different sampleable distribution types.
pub trait Sampleable<T> {
    /// Samples a solution from a sampleable distribution.
    fn sample(&self) -> Vec<T>;
}

/// Trait for learnable populations.
pub trait Learnable<T> {
    /// Learns a probability distribution from a population of solutions.
    fn learn(&self) -> Sampleable<T>;
}
