//! # permu-rs
//!
//! `permu-rs` is a collection of utilities for permutations. It contains useful tools to
//! experiment with permutations, different permutation based problems and
//! bijective-transformations.

use std::error::Error;
use std::fmt;

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
    fn sample(distr: &mut Distribution, out: &mut Self) -> Result<(), Box<dyn Error>>;
}

/// Enum for different probability distribution types. 
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Distribution {
    /// Probability distribution for permutation populations
    PermuDistribution(Vec<Vec<usize>>, bool),
    /// Probability distribution for Vj populations
    VjDistribution(Vec<Vec<usize>>, bool),
}

#[derive(Debug)]
/// Error to return when an incorrect `Distribution` type is given.
pub struct IncorrectDistrType;

impl fmt::Display for IncorrectDistrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incorrect distribution given")
    }
}

impl Error for IncorrectDistrType {}
