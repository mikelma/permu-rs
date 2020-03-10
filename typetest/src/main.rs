use std::fmt::{Display, Debug};
use std::convert::TryFrom;
use std::convert::TryInto;

pub trait NumVec where
    NumVec : Copy +
    Sized +
    From<u8> +
    TryFrom<usize> +
    TryInto<usize> +
    // PartialEq<T> +
    Eq +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    Display + // NOTE : For debugging
    Debug, // NOTE : For debugging
{}

struct Permu {
    inner: Vec<NumVec>,
}

impl Permu {
    fn sum(&self) -> usize {
        self.inner.iter().sum()
    }
    
    fn zeros(len: usize) -> Permu {
        Permu { inner : vec![NumVec::from(0u8); len] }
    }
}

fn main() {
    let permu = Permu { inner: Vec::<u8>::from(vec![0,3,2,5,4]) };
    println!("permu: {}", permu);
}
