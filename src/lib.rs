//! # permu-rs
//!
//! `permu-rs` is a collection of utilities for permutations. It contains useful tools to
//! experiment with permutations, different permutation based problems and
//! bijective-transformations.

use std::error::Error;

// Import modules
pub mod permutation;
pub mod vj;

// Import errors
pub use permutation::NotPermutation;
pub use vj::LengthError;

/// Contains the methods a `Population` should have.
pub trait Population {
    
    /// Returns a `Distribution` learned from the current population.
    fn learn(&self) -> Distribution;

    /// Fills a given `out` population with samples sampled from a given `distr` `Distribution`. 
    fn sample(distr: &mut Distribution, out: &mut Self) -> Result<(), Box<Error>>;
}

/// Probability distribution. 
pub struct Distribution {
    pub distribution : Vec<Vec<usize>>,
    pub soften : bool,
}
