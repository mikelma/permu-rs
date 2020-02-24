use std::convert::{TryFrom, TryInto};
use rand::Rng;

use std::fmt;
use fmt::{Debug, Display};

use crate::permutation::{Permutation, PermuPopulation};
use crate::{Population, Distribution, errors::Error};

/// Contains a Inversion vector and methods to generate and trasnform them.
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
    pub fn from_permu(permu: &Permutation<T>, inversion: &mut Inversion<T>) -> Result<(), Error> {
        
        // Check if sizes are correct
        if permu.permu.len()-1 != inversion.inversion.len() {
            return Err(Error::LengthError);
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
    pub fn to_permu(&self, out: &mut Permutation<T>) -> Result<(), Error> {
         
        // Check if sizes are correct
        if out.permu.len()-1 != self.inversion.len() {
            return Err(Error::LengthError);
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

    /// Creates an `InversionPopulation`based on a given matrix.
    /// # Example
    /// ```
    /// use permu_rs::inversion::InversionPopulation;
    /// let pop: Vec<Vec<u16>> = vec![vec![0,2,0,0], vec![1,2,0,0], vec![0,0,0,0]];
    /// let pop = InversionPopulation::from_vec(&pop).unwrap();
    ///
    /// println!("{}", pop);
    ///
    /// // Now, the seond vector contais one item less 
    /// let pop: Vec<Vec<u16>> = vec![vec![0,2,0,0], vec![1,0,0], vec![0,0,0,0]];
    /// let pop = InversionPopulation::from_vec(&pop); // This should return a LengthError
    /// assert!(pop.is_err());
    /// ```
    pub fn from_vec(vec: &Vec<Vec<T>>) -> Result<InversionPopulation<T>, Error> {
        let mut pop : Vec<Inversion<T>> = Vec::with_capacity(vec.len());

        let len = vec[0].len();

        for v in vec {
            if v.len() == len {
                pop.push(Inversion::from_vec(v.clone()));
            } else {
                return Err(Error::LengthError);
            }
        }

        Ok(InversionPopulation {population: pop, size: vec.len()})
    }

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
    ///
    /// println!("Zeros or identity population\n{}", inversions);
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
    ///
    /// println!("{}\n", inversions);
    /// println!("{}", out_pop);
    /// ```
    pub fn to_permus(&self, permu_pop: &mut PermuPopulation<T>) -> Result<(), Error> {

        // Check if for every Inversion is a Permutation in permu_pop
        if permu_pop.size != self.size {
            return Err(Error::LengthError);
        }

        // Check Permutation and Inversion lengths are compatible
        if permu_pop.population[0].permu.len() != self.population[0].inversion.len()+1 {
            return Err(Error::LengthError);
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
    ///
    /// println!("{}\n", permus);
    /// println!("{}", inversions);
    /// ```
    ///
    pub fn from_permus(permu_pop: &PermuPopulation<T>, 
                       inversions: &mut InversionPopulation<T>) -> Result<(), Error>{
        // Check sizes        
        if permu_pop.size != inversions.size {
            return Err(Error::LengthError);
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

impl<T> Population for InversionPopulation<T> where 
    T : Copy +
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
{
    
    /// Implementation of `learn` method for `InversionPopulation`.
    ///
    /// # Example
    /// ```
    /// use permu_rs::{Population, Distribution};
    /// use permu_rs::inversion::{InversionPopulation, Inversion};
    /// use InversionPopulation as invpop;
    /// use Inversion as inv;
    /// 
    /// let pop: Vec<Vec<u8>> = vec![vec![2,1,0], vec![1,0,0], vec![0,0,0]];
    /// let pop = InversionPopulation::from_vec(&pop).unwrap();
    ///
    /// let target = vec![vec![1,1,1,0],vec![2,1,0,0],vec![3,0,0,0]];
    /// let target = Distribution::InversionDistribution(target, false);
    ///
    /// let distr = pop.learn();
    ///
    /// assert_eq!(target, distr);
    /// ```
    // NOTE: i: positions, j: values
    fn learn(&self) -> Distribution {
        let m = self.population[0].inversion.len();     // Number of positions
        let n = m+1;   // Number of possible values

        let mut distr: Vec<Vec<usize>> = vec![vec![0; n]; m]; // Init distribution matrix
        
        for i in 0..self.population.len() { // For each vector in population
            for j in 0..m { // For position item in the vector
                let value: usize = match self.population[i].inversion[j].try_into() {
                    Ok(val) => val,
                    Err(_) => panic!("Fatal error converting generic type usize"),
                };
                distr[j][value] += 1;
            }
        }
        Distribution::InversionDistribution(distr, false)
    }

    /// Implementation of `sample` method for `PermuPopulation`.
    ///
    /// # Errors
    /// Returns a `LengthError` if the length of the output population's `Inversion`'s length 
    /// is not equal to its population `Inversions`'s. Returns an `IncorrectDistrType` error if
    /// the given distribution is not `InversionPopulation`.
    //
    /// # Example
    /// ```
    /// use permu_rs::{Population, Distribution};
    /// use permu_rs::inversion::InversionPopulation;
    /// 
    /// // Initialize a custom distribution
    /// let distr = vec![vec![1,1,1,0],vec![2,1,0,0],vec![3,0,0,0]];
    /// let mut distr = Distribution::InversionDistribution(distr, false);
    /// println!("Original distr:\n{}", distr);
    /// // Init output population
    /// let mut out = InversionPopulation::<u8>::zeros(10, 3); 
    /// // Sample distribution
    /// InversionPopulation::sample(&mut distr, &mut out).unwrap();
    ///
    /// // Now the original distribution has been changed in order to soften it
    /// println!("Now distr:\n{}", distr);
    /// println!("Out:\n{}", out); // Sampled population
    /// ```
    fn sample(distr: &mut Distribution, out: &mut Self) -> Result<(), Error> {
        // Check if the given Distribution type is correct
        let (distr, soften) = match distr {
            Distribution::InversionDistribution(d, s) => (d, s),
            _ => return Err(Error::IncorrectDistrType), 
        };

        // Check distribution and population's vector's sizes are correct
        let length = match distr.len() == out.population[0].inversion.len() {
            true => distr.len(),
            false => return Err(Error::LengthError),
        };
         
        // Check if the distribution is soften
        if !*soften {
            // If not, soften the distribution by adding one to every element of the matrix.
            // In this case, only the elements in the upper diagonal of the matrix are modified.
            let mut max_val = length+1;
            (0..length).for_each(|i| {
                (0..length+1).for_each(|j| {
                    if j < max_val {
                            distr[i][j] += 1;
                    } 
                });
                max_val -= 1;
            });
            // Mark the distribution as soften
            *soften = true;
        }

        (0..out.size).for_each(|out_i| { // For each individual in the population (out_i=index)

            // Iterate the distribution randomly
            Permutation::<usize>::random(length).permu.iter()
                .for_each(|pos_i| { // For each row in the distribution (random) 
                    let max_sum : usize = distr[*pos_i].iter().sum();
                    let rand: f64 = rand::thread_rng().gen_range(0.0, max_sum as f64);
                    
                    let mut sum = distr[*pos_i][0]; // Sum is initialized with the first value of distr[pos_i]
                    let mut i = 0;
                    while (sum as f64) < rand {
                        i += 1;
                        sum += distr[*pos_i][i];
                    }

                    // Add sampled value to the individual that is being sampled
                    out.population[out_i].inversion[*pos_i] = match T::try_from(i) {
                        Ok(v) => v,
                        Err(_) => panic!("Fatal conversion error"), // NOTE: Properly panic
                    };
                });
        });
        Ok(())
    }
}

impl<T> fmt::Display for InversionPopulation<T> where 
    T : Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        // For empty distibutions
        if self.size == 0 {
            return write!(f, "[]\nInversionPopulation. Shape: 0,0\n");
        }

        let mut formatted = String::from("[");

        self.population.iter()
            .take(self.size -1) // Do not take the last item
            .for_each(|inv| {
                formatted.push_str(format!("{:?},\n", inv.inversion).as_str());
            });

        // Now, take the last item
        formatted.push_str(format!("{:?}]", 
                                   self.population[self.size-1].inversion).as_str());

        write!(f, "{}\nInversionPopulation. Shape: {},{}\n", 
               formatted, self.size, self.population[0].inversion.len())
    }
}
