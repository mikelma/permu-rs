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
    
    /// Creates a `Rim`vector of the length given.
    pub fn zeros(length: usize) -> Rim<T> {
        Rim { inner: vec![T::from(0u8); length] }
    }
    
    /// Returns the length of the inner `Rim` vector.
    pub fn len(self) -> usize {
        self.inner.len()
    }
    
    /// Transforms a given insertion vector (RIM) into it's permutation representation. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::{ permutation::Permutation, rim::Rim };
    /// let rim = Rim::<u8>::from_vec(vec![0,2,2]);
    /// let mut output = Permutation::<u8>::identity(4);
    ///
    /// Rim::<u8>::to_permu(&rim, &mut output).unwrap();
    ///
    /// println!("insertion vector: {:?}", rim.inner);
    /// println!("permutation: {:?}", output.permu);
    ///
    /// let target = Permutation::from_vec(vec![1,0,3,2]).unwrap();
    /// assert_eq!(target, output);
    ///
    /// ```
    pub fn to_permu(iv: &Rim<T>, out: &mut Permutation<T>) -> Result<(), Error> {
        let permu_length = iv.inner.len()+1;
        // Clear all the values from the output permutation
        out.permu.clear();
        let inner = &mut out.permu;

        // Start by pushing 0 to the output permutation 
        inner.push(T::from(0u8));
    
        (1..permu_length)
            .for_each(|e| {
                // Get the index to insert the element
                let index = match iv.inner[e-1].try_into() {
                        Ok(v) => {
                            if v > inner.len() {
                                inner.len()
                            } else {
                                v
                            }
                        },
                        Err(_) => panic!("Fatal conversion error"),
                };
                // Obtain the element to insert (from identity)
                let element = match T::try_from(e) {
                        Ok(v) => v,
                        Err(_) => panic!("Fatal conversion error"),
                };
                
                inner.insert(index, element);
            });
        Ok(())
    }

    /// Transforms a given permutation vector into it's insertion vector (Rim) representation. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::{ permutation::Permutation, rim::Rim };
    /// let permu = Permutation::<u8>::from_vec(vec![1,0,3,2]).unwrap();
    /// let mut rim = Rim::<u8>::zeros(3);
    ///
    /// Rim::<u8>::from_permu(&permu, &mut rim).unwrap();
    ///
    /// println!("permutation: {:?}", permu.permu);
    /// println!("insertion vector: {:?}", rim.inner);
    ///
    /// let target = Rim::<u8>::from_vec(vec![0,2,2]);
    /// assert_eq!(target, rim);
    /// ```
    pub fn from_permu(permu: &Permutation<T>, out: &mut Rim<T>) -> Result<(), Error> {
        let mut permu = permu.permu.clone(); // NOTE: Not efficient
        let length = permu.len();
        // let mut inner: Vec<T> = vec![T::from(0u8); length];
        out.inner = out.inner.iter_mut()
                             .map(|_| T::from(0u8))
                             .collect();

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
                        Err(_) => unreachable!(),
                    },
                    None => unreachable!(),
                };

                //println!("Position of {} is {}", element, index_t);
                out.inner[element-1] = index_t;

                permu.remove(index);

            });

        Ok(())
    }
}

/// Population of `Rim` vectors.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct RimPopulation<T> {
    pub population : Vec<Rim<T>>,
    pub size : usize,
}

impl<T> RimPopulation<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    TryInto<usize> +
    // PartialEq<T> +
    Eq +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    // Into<usize> +
    Display + // NOTE : For debugging
    Debug, // NOTE : For debugging
{
}
