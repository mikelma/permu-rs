use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display};
use std::fmt;

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
    /// Creates an `InversionPopulation` based on a given matrix.
    /// # Example
    /// ```
    /// use permu_rs::rim::RimPopulation;
    /// let pop: Vec<Vec<u16>> = vec![vec![0,2,0,0], vec![1,2,0,0], vec![0,0,0,0]];
    /// let pop = RimPopulation::from_vec(&pop).unwrap();
    ///
    /// println!("{}", pop);
    ///
    /// // Now, the seond vector contais one item less 
    /// let pop: Vec<Vec<u16>> = vec![vec![0,2,0,0], vec![1,0,0], vec![0,0,0,0]];
    /// let pop = RimPopulation::from_vec(&pop); // This should return a LengthError
    /// assert!(pop.is_err());
    /// ```
    pub fn from_vec(vec: &Vec<Vec<T>>) -> Result<RimPopulation<T>, Error> {
        let mut pop : Vec<Rim<T>> = Vec::with_capacity(vec.len());
        let len = vec[0].len();

        for v in vec {
            if v.len() == len {
                pop.push(Rim::from_vec(v.clone()));
            } else {
                return Err(Error::LengthError);
            }
        }
        Ok(RimPopulation {population: pop, size: vec.len()})
    }
}

impl<T> fmt::Display for RimPopulation<T> where 
    T : Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        // For empty distibutions
        if self.size == 0 {
            return write!(f, "[]\nRimPopulation. Shape: 0,0\n");
        }

        let mut formatted = String::from("[");
        self.population.iter()
            .take(self.size -1) // Do not take the last item
            .for_each(|rim| {
                formatted.push_str(format!("{:?},\n", rim.inner).as_str());
            });

        // Now, take the last item
        formatted.push_str(format!("{:?}]", 
                                   self.population[self.size-1].inner).as_str());

        write!(f, "{}\nInversionPopulation. Shape: {},{}\n", 
               formatted, self.size, self.population[0].inner.len())
    }
}
