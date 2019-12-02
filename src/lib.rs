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

/// Probability distribution. 
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Distribution {
    PermuDistribution(Vec<Vec<usize>>, bool),
    VjDistribution(Vec<Vec<usize>>, bool),
}
/*
impl Distribution {

    pub fn get_distribution(&self) -> Result<RefCell<Vec<Vec<usize>>>, EmptyDistribution> {

        match self {
            Distribution::PermuPopulation(distr, _) => RefCell::new(distr),
        }
    }
}
*/

#[derive(Debug)]
pub struct IncorrectDistrType;

impl fmt::Display for IncorrectDistrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incorrect distribution given")
    }
}

impl Error for IncorrectDistrType {}
