use std::convert::TryFrom;
use rand::Rng;
use std::fmt::{Debug, Display};

/// Contains a Vj vector.
#[derive(Debug)]
pub struct Vj<T> {
    pub vj : Vec<T>,
}

impl<T> Vj<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    PartialEq<T> +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    Display + // NOTE : For debugging
    Debug // NOTE : For debugging
{

    /// Creates a Vj object from the vector.
    pub fn from_vec(vec : Vec<T>) {
        
    }
}
