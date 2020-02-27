use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display};
use std::fmt;

use crate::errors::Error;
use crate::permutation::{Permutation, PermuPopulation};

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
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Transforms a given insertion vector (RIM) into it's permutation representation. 
    ///
    /// # Errors
    /// Returns a `LengthError` if the length of the output permutation is not the length of the
    /// given rim vector + 1.
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
    /// ```
    pub fn to_permu(iv: &Rim<T>, out: &mut Permutation<T>) -> Result<(), Error> {
        let permu_length = out.len();
        // Check lengths are compatible
        if permu_length != iv.len() + 1 {
            return Err(Error::LengthError);     
        }

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
    /// # Errors
    /// Returns a `LengthError` if the length of the given permutation is not the length of the
    /// output rim vector + 1.
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
        let length = permu.len();
        // Check lengths
        if length != out.len() + 1 {
            return Err(Error::LengthError);     
        }

        let mut permu = permu.permu.clone(); // NOTE: Not efficient
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
    ///
    /// # Errors
    /// Returns a `LengthError` if the length of all vectors is not equal.
    ///
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
    
    /// Creates a `RimPopulation` of zero valued `Rim` vectors of the size and length given.
    ///
    /// # Example
    /// ```
    /// use permu_rs::rim::RimPopulation;
    ///
    /// let (size, length) = (7, 5);
    /// let pop = RimPopulation::<u8>::zeros(size, length);
    /// println!("{}", pop);
    /// ```
    pub fn zeros(size: usize, length: usize) -> RimPopulation<T> {
        let mut population: Vec<Rim<T>> = Vec::with_capacity(size); 
        let zeros = vec![T::from(0u8);length];

        (0..size).for_each(|_| population.push(Rim::from_vec(zeros.clone())));
        
        RimPopulation { population, size }
    }
    
    /// Takes a `PermuPopulation` as an output and fills this population with the `Permutation` 
    /// representation  of each `Rim`vector in the `RimPopulation`. `RimPopulation` to its 
    /// `Permutation` representation. Positions of vectors are respected.
    ///
    /// # Errors
    /// Returns a `LengthError` if the size of both population isn't equal or the length
    /// of the `Permutation`s isn't the length of the `Rim` vectors + 1.
    ///
    /// # Example
    /// ```
    /// use permu_rs::{
    ///     permutation::{ Permutation, PermuPopulation },
    ///     rim::RimPopulation };
    ///
    /// let (size, length) = (10, 5);
    /// let rim_zeros = RimPopulation::<u16>::zeros(size, length-1);
    /// let mut permus = PermuPopulation::<u16>::random(size, length);
    /// // The output should look like this
    /// let target = PermuPopulation::<u16>::from_vec(vec![
    ///                 Permutation::<u16>::from_vec(vec![4,3,2,1,0]).unwrap();size]);
    ///
    /// // Convert the rim population to its permutation representation 
    /// rim_zeros.to_permus(&mut permus).unwrap();
    /// assert_eq!(target, permus);
    /// ```
    pub fn to_permus(&self, permu_pop: &mut PermuPopulation<T>) -> Result<(), Error> {

        // Check if for every Rim vector there's a Permutation in permu_pop
        if permu_pop.size != self.size {
            return Err(Error::LengthError);
        }

        // Check Permutation and Rim vector lengths are compatible
        if permu_pop.population[0].len() != self.population[0].len()+1 {
            return Err(Error::LengthError);
        }
        
        // Convert each Rim vector of the population to permutation 
        (0..self.size).for_each(|i| {
            match Rim::to_permu(&self.population[i], &mut permu_pop.population[i]) {
                Ok(_) => (),
                Err(e) => panic!("Fatal error converting InversionPopulation to PermuPopulation: {}", e),
            }
        });
        Ok(())
    }
    
     
    /// Fills a `RimPopulation` with the rim vector representation of each 
    /// permutation vector inside a given `PermuPopulation`. Note that the sizes
    /// of both populations must match and the length of permutations must be 
    /// equal to the length of the rim vectors + 1.
    ///
    /// # Errors
    /// Returns a `LengthError` if the size of both population isn't equal or the length
    /// of the `Permutation`s isn't the length of the `Rim` vectors + 1.
    ///
    /// # Example
    /// ```
    /// use permu_rs::{
    ///     permutation::PermuPopulation,
    ///     rim::RimPopulation,
    /// };
    /// // Create a target population of random permutations
    /// let mut permus = PermuPopulation::<u8>::random(10, 5);
    /// let target = permus.clone();
    /// // Init rim population
    /// let mut rims = RimPopulation::<u8>::zeros(10, 4);
    ///
    /// // Convert the permutations into rim vectors and then recover the 
    /// // original permutations from the rim vectors.
    /// RimPopulation::from_permus(&permus, &mut rims).unwrap();
    /// RimPopulation::to_permus(&rims, &mut permus).unwrap();
    ///
    /// assert_eq!(target, permus);
    /// ```
    pub fn from_permus(permu_pop: &PermuPopulation<T>, 
                       rim_pop: &mut RimPopulation<T>) -> Result<(), Error>{
        // Check sizes        
        if permu_pop.size != rim_pop.size {
            return Err(Error::LengthError);
        }
        // Check lengths, permu.len() must be rim.len()+1
        if permu_pop.population[0].len() != rim_pop.population[0].len()+1 {
            return Err(Error::LengthError);
        }

        permu_pop.population.iter()
            .enumerate()
            .for_each(|(i, permu)| Rim::from_permu(permu, &mut rim_pop.population[i]).unwrap());

        Ok(())
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
