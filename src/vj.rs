use std::convert::TryFrom;
use rand::Rng;
use std::fmt::{Debug, Display};

use crate::permutation;

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
    ///
    /// # Example
    /// ```
    /// use permu_rs::vj::Vj;
    /// let vj_vec = vec![0,0,1,1,4];
    /// let my_vj =  Vj::from_vec(vj_vec);
    /// ```
    pub fn from_vec(vec : Vec<T>) -> Vj<T> {
        Vj { vj : vec }        
    }

    /// Creates a Vj filled with 0s. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::vj::Vj;
    /// assert_eq!(vec![0,0,0], Vj::zeros(3).vj);
    /// ```
    pub fn zeros(lenght: usize) -> Vj<T> {
        Vj { vj : vec![T::from(0u8); lenght] }
    }
    
    /// Fills a given `Vj` with the vj representation of the given `Permutation`.
    ///
    /// # Errors
    /// The length of the `Vj` must be the size of the `Permutation` - 1. Otherwise, 
    /// the function will return an error.
    ///
    /// # Example
    /// ```
    /// use permu_rs::*;
    /// let permu = permutation::Permutation::<u8>::from_vec(vec![0,3,2,1]).unwrap();
    /// let mut vj_repr = vj::Vj::zeros(3);
    /// vj::Vj::transform_from_permu(&permu, &mut vj_repr).unwrap();
    /// assert_eq!(vec![0,2,1], vj_repr.vj);
    /// ```
    pub fn transform_from_permu(permu: &permutation::Permutation<T>, vj: &mut Vj<T>) -> Result<(), &'static str>{
        
        // Check if sizes are correct
        if permu.permu.len()-1 != vj.vj.len() {
            return Err("Lenght of the vj vector must be permu.len()-1");
        }

        for index in 0..vj.vj.len() {

            let mut n = 0;
            for i in index..permu.permu.len() {

                if permu.permu[index] > permu.permu[i] {
                    n += 1;
                }            

                // This will never fail, as the boundaries of T are always respected
                vj.vj[index] = match T::try_from(n) {
                    Ok(v) => v,
                    Err(_) => return Err("Error while coverting usize to T"),
                };
            }
        }
        Ok(())
    } 
}
