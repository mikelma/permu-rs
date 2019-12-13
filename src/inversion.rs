use std::convert::{TryFrom, TryInto};
use rand::Rng;

use std::fmt;
use fmt::{Debug, Display};

use std::error::Error;

use crate::permutation;

/// Contains a Inversion vector and method to generate and trasnform them.
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Inversion<T> {
    pub inversion : Vec<T>,
}

impl<T> Inversion<T> where 
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
    /// use permu_rs::inversion::Inversion;
    /// let inversion_vec = vec![0,0,1,1,4];
    /// let my_inversion = Inversion::<u8>::from_vec(inversion_vec);
    /// ```
    pub fn from_vec(vec : Vec<T>) -> Inversion<T> {
        Inversion { inversion : vec }        
    }

    /// Creates a Inversion filled with 0s. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::inversion::Inversion;
    /// assert_eq!(vec![0,0,0], Inversion::<u8>::zeros(3).inversion);
    /// ```
    pub fn zeros(length: usize) -> Inversion<T> {
        Inversion { inversion : vec![T::from(0u8); length] }
    }
    
    /// Fills a given `Inversion` with the inversion representation of the given `Permutation`.
    ///
    /// # Errors
    /// The length of the `Inversion` must be the size of the `Permutation` - 1. Otherwise, 
    /// the function will return a `LengthError`.
    ///
    /// # Example
    /// ```
    /// use permu_rs::*;
    /// let permu = permutation::Permutation::<u8>::from_vec(vec![0,3,2,1]).unwrap();
    /// let mut inversion_repr = inversion::Inversion::zeros(3);
    /// inversion::Inversion::from_permu(&permu, &mut inversion_repr).unwrap();
    /// assert_eq!(vec![0,2,1], inversion_repr.inversion);
    /// ```
    pub fn from_permu(permu: &permutation::Permutation<T>, inversion: &mut Inversion<T>) -> Result<(), LengthError>{
        
        // Check if sizes are correct
        if permu.permu.len()-1 != inversion.inversion.len() {
            return Err(LengthError::new());
        }

        for index in 0..inversion.inversion.len() {

            let mut n = 0;
            for i in index..permu.permu.len() {

                if permu.permu[index] > permu.permu[i] {
                    n += 1;
                }            

                // This will never fail, as the boundaries of T are always respected
                inversion.inversion[index] = match T::try_from(n) {
                    Ok(v) => v,
                    Err(_) => panic!("Fatal conversion error"),
                };
            }
        }
        Ok(())
    } 

    /// Returns a `Permutation` created from the `Inversion` representation.
    ///
    /// # Errors
    /// The length of the `Inversion` must be the size of the `Permutation` - 1. Otherwise, 
    /// the function will return a `LengthError` error.
    ///
    /// # Example
    /// ```
    /// use permu_rs::*;
    /// let inversion = inversion::Inversion::<u8>::from_vec(vec![0,2,1]);
    /// let mut permu = permutation::Permutation::<u8>::identity(4);
    /// inversion.to_permu(&mut permu).unwrap();
    /// assert_eq!(vec![0,3,2,1], permu.permu);
    /// ```
    pub fn to_permu(&self, out: &mut permutation::Permutation<T>) -> Result<(), LengthError> {
         
        // Check if sizes are correct
        if out.permu.len()-1 != self.inversion.len() {
            return Err(LengthError::new());
        }

        let permu = &mut out.permu;
        let inversion = &self.inversion;
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

        inversion.iter().chain([T::from(0u8)].iter()) // Create a Inversion iterator and append 0 element to it
            .enumerate()
            .for_each(|(index, inversion_val)| {

                // Get the value and index of element in e[inversion_val]
                let value = e.iter()
                    .enumerate()
                    .find(|(i, _)| *inversion_val == match T::try_from(*i) {
                        Ok(v) => v,
                        Err(_) => panic!("fatal conversion error"),
                    });
                
                // This will never fail as the boundaries of T are always respected here
                let (remove_index, value) = match value {
                    Some(a) => a,
                    None => panic!("Fatal conversion error"),
                };
                
                permu[index] = *value;
                e.remove(remove_index);
            });

        Ok(())
    } 
}

/// Population of Inversion objects. Includes initilializers and transformation tools.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct InversionPopulation<T> {
    pub population : Vec<Inversion<T>>,
    pub size : usize,
}

impl<T> InversionPopulation<T> where 
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
    /// Creates a `InversionPopulation` of the size given with `Inversion`s of length specified, filled with 0s. 
    /// This population represents a population of identity permutations.
    ///
    /// # Example
    /// ```
    /// use permu_rs::*;
    /// use permutation::{Permutation, PermuPopulation};
    /// use inversion::{Inversion, InversionPopulation};
    ///
    /// let (size, length) = (20,10);
    /// let identity = PermuPopulation::from_vec(vec![Permutation::<u8>::identity(length);size]);
    /// let inversions = InversionPopulation::<u8>::zeros(size,length-1);
    /// let mut permus = PermuPopulation::<u8>::zeros(size, length);
    ///
    /// inversions.to_permus(&mut permus);
    /// assert_eq!(identity, permus);
    /// ```
    pub fn zeros(size: usize, length: usize) -> InversionPopulation<T> {
        let mut population: Vec<Inversion<T>> = Vec::with_capacity(size); 
        let zeros = vec![T::from(0u8);length];

        (0..size).for_each(|_| population.push(Inversion::from_vec(zeros.clone())));
        
        InversionPopulation { population, size }
    }
    
    /// Transforms the `Inversion` to its `Permutation` representation. Fills a given `PermuPopulation`
    /// based on the `Inversion`s from the `InversionPopulation`. The `Inversion` -> `Permutation` transformation is 
    /// done respecting the positions in the population.
    ///
    /// # Errors
    /// Returns a `LengthError` if the size of both `Populations` are not equal. 
    ///
    /// # Panics
    /// The mothod will panic if a `Inversion` of the `InversionPopulation` has not a `Permutation`
    /// representation.
    ///
    /// # Example
    /// ```
    /// use permu_rs::*;
    /// let (size, length) = (5, 10);
    ///
    /// let mut out_pop = permutation::PermuPopulation::<u8>::zeros(size, length); // Output permutation
    ///
    /// let identity_pop = permutation::PermuPopulation::<u8>::identity(size, length);
    /// let inversions = inversion::InversionPopulation::<u8>:: zeros(size, length-1);
    ///
    /// inversions.to_permus(&mut out_pop);
    ///
    /// assert_eq!(out_pop, identity_pop);
    /// ```
    pub fn to_permus(&self, permu_pop: &mut permutation::PermuPopulation<T>) -> Result<(), LengthError> {

        // Check if for every Inversion is a Permutation in permu_pop
        if permu_pop.size != self.size {
            return Err(LengthError::from(String::from(
                "InversionPopulation and the given PermuPopulation sizes must be equal")));
        }

        // Check Permutation and Inversion lengths are compatible
        if permu_pop.population[0].permu.len() != self.population[0].inversion.len()+1 {
            return Err(LengthError::from(String::from(
                "The length of Permutations from PermuPopulation must be the length of Inversions+1")));
        }
        
        // Convert each Inversion of the population to permutation 
        (0..self.size).for_each(|i| {
            match self.population[i].to_permu(&mut permu_pop.population[i]) {
                Ok(_) => (),
                Err(e) => panic!("Fatal error converting InversionPopulation to PermuPopulation: {}", e),
            }
        });
        Ok(())
    }
    
    /// Fills a given `InversionPopulation` with the `inversion` representations from a 
    /// `PermuPopulation`. The transformation is done respecting the positions inside the 
    /// `PermuPopulation`.
    /// 
    /// # Errors
    /// Returns a `LengthError` if the size of both populations are not equal.
    ///
    /// # Panics 
    /// The function panics if the internal `Inversion::from_permu` returns an `Error`.
    /// This will happen if a type conversion error occurs.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::{Permutation, PermuPopulation};
    /// use permu_rs::inversion::{Inversion, InversionPopulation};
    ///
    /// let (size, length) = (5, 4);
    ///
    /// let mut population = vec![Inversion::<u16>::from_vec(vec![1,0,0]); size];
    /// let mut inversions = InversionPopulation{ population, size };
    ///
    /// let inversion_ok = InversionPopulation::<u16>::zeros(size, length-1); // Correct result
    /// let permus = PermuPopulation::<u16>::identity(size, length);
    ///
    /// InversionPopulation::from_permus(&permus, &mut inversions);
    /// assert_eq!(inversion_ok, inversions);
    /// ```
    ///
    pub fn from_permus(permu_pop: &permutation::PermuPopulation<T>, 
                       inversions: &mut InversionPopulation<T>) -> Result<(), LengthError>{
        // Check sizes        
        if permu_pop.size != inversions.size {
            return Err(LengthError::from(String::from(
                "InversionPopulation's and  PermuPopulation's sizes must be equal")));
        }

        permu_pop.population.iter()
            .enumerate()
            .for_each(|(i, permu)| { match Inversion::from_permu(permu, &mut inversions.population[i]) {
                Ok(_) => (),
                Err(e) => panic!(e),
            }});

        Ok(())
    }
}

/// Error type to return when transforming between representations and the 
/// length of one of the vectors is not correct
#[derive(Debug)]
pub struct LengthError {
    message: Option<String>,
}

impl LengthError {

    /// Creates a `LengthError` object with the default error messge.
    /// # Example
    /// ```
    /// use permu_rs::LengthError;
    /// let my_error = LengthError::new();
    /// ```
    pub fn new() -> LengthError {
        LengthError { message : None }
    }
    
    /// Creates a `LengthError` object including a given custom error message.
    /// # Example
    /// ```
    /// use permu_rs::LengthError;
    /// let my_error = LengthError::from(String::from("Super custom message"));
    /// ```
    pub fn from(m: String) -> LengthError {
        LengthError { message : Some(m) }
    }
}

impl fmt::Display for LengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(m) => write!(f, "{}", m),
            None => write!(f, "Please check the lengths or shapes of the given arguments"),
        }
    }
}

impl Error for LengthError {}
