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
        let mut v = Vec::with_capacity(lenght);
        (0..lenght).for_each(|_| v.push(T::from(0u8)));
        Vj { vj : v }
    }

    /*
    pub fn from_permu(permu: &permutation::Permutation<T>, vj: &mut Vj<T>) -> Result<(), &'static str>{
        
        assert_eq!(permu.len()-1, vj.len(), "Lenght of the vj vector must be permu.len()-1");
        
        // Reset vj
        for i in 0..vj.len() {
            vj[i] = 0;
        }

        for indx in 0..permu.len() {
            for i in indx..permu.len() {
                if permu[i] < permu[indx] {
                    vj[indx] += 1;
                }            
            }
        }
        Ok(())
    } 
    */
}
