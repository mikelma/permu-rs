use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display};

use crate::errors::Error;
use crate::permutation::{Permutation};

/// Contains a repeated insertion model (RIM) vector and methods to generate and trasnform them.
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Rim<T> {
    pub inner : Vec<T>,
}

impl<T> Rim<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    TryInto<usize> +
    //PartialEq<T> +
    Eq +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    // Into<usize> +
    Display + // NOTE : For debugging
    Debug // NOTE : For debugging
{
    /// Creates a Inversion object from the vector.
    ///
    /// # Example
    /// ```
    /// use permu_rs::rim::Rim;
    /// let rim_vec = vec![0,0,1,1];
    /// let rim = Rim::<u8>::from_vec(rim_vec);
    /// ```
    pub fn from_vec(inner : Vec<T>) -> Rim<T> {
        Rim { inner }        
    }
    
    /// Transforms a given insertion vector (RIM) into it's permutation representation. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::{ permutation::Permutation, rim::Rim };
    /// let rim = Rim::<u8>::from_vec(vec![0,0,2,2]);
    /// let permu = Rim::<u8>::to_permu(&rim).unwrap();
    /// println!("insertion vector: {:?}", rim.inner);
    /// println!("permutation: {:?}", permu.permu);
    ///
    /// let target = Permutation::from_vec(vec![1,0,3,2]).unwrap();
    /// assert_eq!(target, permu);
    ///
    /// ```
    pub fn to_permu(iv: &Rim<T>) -> Result<Permutation<T>, Error> {
        let length = iv.inner.len();
        let mut inner : Vec<T> = vec![iv.inner[0]];
    
        (1..length)
            .for_each(|e| {

                let index = match iv.inner[e].try_into() {
                        Ok(v) => {
                            if v > inner.len() {
                                inner.len()
                            } else {
                                v
                            }
                        },
                        Err(_) => panic!("Fatal conversion error"),
                };

                let element = match T::try_from(e) {
                        Ok(v) => v,
                        Err(_) => panic!("Fatal conversion error"),
                };
                
                inner.insert(index, element);
                // println!("*inner: {:?}, insert {} in {}", inner, e, index);
            });

        Ok(Permutation::from_vec(inner)?)
    }

    /// Transforms a given permutation vector into it's insertion vector (Rim) representation. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::{ permutation::Permutation, rim::Rim };
    /// let permu = Permutation::<u8>::from_vec(vec![1,0,3,2]).unwrap();
    /// let rim = Rim::<u8>::from_permu(&permu).unwrap();
    ///
    /// println!("permutation: {:?}", permu.permu);
    /// println!("insertion vector: {:?}", rim.inner);
    ///
    /// let target = Rim::<u8>::from_vec(vec![0,0,2,2]);
    /// assert_eq!(target, rim);
    /// ```
    pub fn from_permu(permu: &Permutation<T>) -> Result<Rim<T>, Error> {
        let mut permu = permu.permu.clone(); // NOTE: Not efficient
        let length = permu.len();
        let mut inner: Vec<T> = vec![T::from(0u8); length];

        (1..length).rev()
            .for_each(|element| {

                let elem_t = match T::try_from(element) {
                    Ok(v) => v,
                    Err(_) => unimplemented!(),
                };

                let index = permu.iter().position(|&e| e == elem_t);

                let (index_t, index) = match index {
                    Some(i) => match T::try_from(i) {
                        Ok(v) => (v, i),
                        Err(_) => unimplemented!(),
                    },
                    None => unreachable!(),
                };

                //println!("Position of {} is {}", element, index_t);
                inner[element] = index_t;

                permu.remove(index);

            });

        Ok(Rim::from_vec(inner))
    }
}
