use std::convert::{TryFrom, TryInto};
use rand::Rng;
use std::fmt::{Debug, Display};

use crate::permutation;

/// Contains a Vj vector.
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Vj<T> {
    pub vj : Vec<T>,
}

impl<T> Vj<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    //PartialEq<T> +
    Eq +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    Into<usize> +
    Display + // NOTE : For debugging
    Debug // NOTE : For debugging
{

    /// Creates a Vj object from the vector.
    ///
    /// # Example
    /// ```
    /// use permu_rs::vj::Vj;
    /// let vj_vec = vec![0,0,1,1,4];
    /// let my_vj = Vj::<u8>::from_vec(vj_vec);
    /// ```
    pub fn from_vec(vec : Vec<T>) -> Vj<T> {
        Vj { vj : vec }        
    }

    /// Creates a Vj filled with 0s. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::vj::Vj;
    /// assert_eq!(vec![0,0,0], Vj::<u8>::zeros(3).vj);
    /// ```
    pub fn zeros(length: usize) -> Vj<T> {
        Vj { vj : vec![T::from(0u8); length] }
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
    /// vj::Vj::from_permu(&permu, &mut vj_repr).unwrap();
    /// assert_eq!(vec![0,2,1], vj_repr.vj);
    /// ```
    pub fn from_permu(permu: &permutation::Permutation<T>, vj: &mut Vj<T>) -> Result<(), &'static str>{
        
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

    /// Returns a `Permutation` created from the `Vj` representation.
    ///
    /// # Errors
    /// The length of the `Vj` must be the size of the `Permutation` - 1. Otherwise, 
    /// the function will return an error.
    ///
    /// # Example
    /// ```
    /// use permu_rs::*;
    /// let vj = vj::Vj::<u8>::from_vec(vec![0,2,1]);
    /// let mut permu = permutation::Permutation::<u8>::identity(4);
    /// vj.to_permu(&mut permu).unwrap();
    /// assert_eq!(vec![0,3,2,1], permu.permu);
    /// ```
    pub fn to_permu(&self, out: &mut permutation::Permutation<T>) -> Result<(), &'static str> {
         
        // Check if sizes are correct
        if out.permu.len()-1 != self.vj.len() {
            return Err("Lenght of the vj vector must be permu.len()-1");
        }

        let permu = &mut out.permu;
        let vj = &self.vj;
        let size = permu.len();
        
        // Create T identity
        let mut e: Vec<T> = Vec::with_capacity(size);
        (0..size).for_each(|v| { 
            // This will never fail as the boundaries of T are always respected here
            e.push(match T::try_from(v) {
                Ok(a) => a,
                Err(_) => panic!("Conversion Infallible error"),
            }) 
        });

        vj.iter().chain([T::from(0u8)].iter()) // Create a Vj iterator and append 0 element to it
            .enumerate()
            .for_each(|(index, vj_val)| {

                // Get the value and index of element in e[vj_val]
                let value = e.iter()
                    .enumerate()
                    .find(|(i, _)| *vj_val == match T::try_from(*i) {
                        Ok(v) => v,
                        Err(_) => panic!("This should not fail"),
                    });
                
                // This will never fail as the boundaries of T are always respected here
                let (remove_index, value) = match value {
                    Some(a) => a,
                    None => panic!("Conversion error"),
                };
                
                permu[index] = *value;
                e.remove(remove_index);
            });

        Ok(())
    } 
}

#[derive(PartialEq)]
pub struct VjPopulation<T> {
    pub population : Vec<Vj<T>>,
    pub size : usize,
}

impl<T> VjPopulation<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    // PartialEq<T> +
    Eq +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    Into<usize> +
    Display + // NOTE : For debugging
    Debug, // NOTE : For debugging
{
    /// Creates a `VjPopulation` of the size given with `Vj`s of length specified, filled with 0s. 
    /// This population represents a population of identity permutations.
    ///
    /// # Example TODO
    /// ```
    /// use permu_rs::*;
    /// use permutation::{Permutation, PermuPopulation};
    /// use vj::{Vj, VjPopulation};
    /// /*
    /// let (size, length) = (20,10);
    /// let identity = PermuPopulation::from_vec(nvec![Permutation::<u8>::identity(length);size]);
    /// let vjs = VjPopulation::<u8>::zeros(size,length);
    /// let permus = PermuPopulation::<u8>::zeros(size, length);
    /// vjs.to_permu(&mut permus);
    /// assert_eq!(identity.population, permus.population);
    /// */
    /// ```
    pub fn zeros(size: usize, length: usize) -> VjPopulation<T> {
        let mut population: Vec<Vj<T>> = Vec::with_capacity(size); 
        let zeros = vec![T::from(0u8);length];

        (0..size).for_each(|_| population.push(Vj::from_vec(zeros.clone())));
        
        VjPopulation { population, size }
    }

    pub fn to_permu(&self, permu_pop: &mut permutation::PermuPopulation<T>) -> Result<(), &'static str> {

        // Check if for every Vj is a Permutation in permu_pop
        if permu_pop.size != self.size {
            return Err("VjPopulation and the given PermuPopulation sizes must be equal");
        }
        
        // Convert each Vj of the population to permutation 
        (0..self.size).for_each(|i| {
            match self.population[i].to_permu(&mut permu_pop.population[i]) {
                Ok(_) => (),
                Err(e) => panic!("Fatal error converting VjPopulation to PermuPopulation: {}", e),
            }
        });
        Ok(())
    }
}

