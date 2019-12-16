//! # permu-rs
//!
//! `permu-rs` is a collection of utilities for permutations. It contains useful tools to
//! experiment with permutations, different permutation based problems and
//! bijective-transformations.

use std::error::Error;
use std::fmt;

// Import modules
pub mod permutation;
pub mod inversion;

// Import errors
pub use permutation::NotPermutation;
pub use inversion::LengthError;

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
    /// Probability distribution for Inversion populations
    InversionDistribution(Vec<Vec<usize>>, bool),
}

impl fmt::Display for Distribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let (distr, soften, distr_type) = match self {
            Distribution::PermuDistribution(v, s) => (v, s, "PermuDistribution"),
            Distribution::InversionDistribution(v, s) => (v, s, "InversionDistribution"),
        };

        // For empty distibutions
        if distr.len() == 0 {
            return write!(f, "[]\n");
        }

        let mut formatted = String::from("[");

        distr.iter()
            .take(distr.len() -1) // Do not take the last item
            .for_each(|row| {
                formatted.push_str(format!("{:?},\n", row).as_str());
            });

        // Now, take the last item
        formatted.push_str(format!("{:?}]", 
                                   distr[distr.len()-1]).as_str());

        write!(f, "{}\n{}, soften: {}\n", formatted, distr_type, soften)
    }
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
