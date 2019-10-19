use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::{Debug, Display};

use rand::Rng;

use crate::Population;

/// Contains a permutation vector methods to generate permutations.
#[derive(Debug)]
pub struct Permutation<T> {
    pub permu : Vec<T>,
}

impl<T> Permutation<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    TryInto<usize> +
    PartialEq<T> +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    Display + // NOTE : For debugging
    Debug, // NOTE : For debugging
{
    
    /// Initializes a Permutation with the given vector. 
    ///
    /// # Errors
    /// If the given vector is not a permutation the function will return an Error. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::Permutation;
    /// let vec : Vec<u16> = vec![0,1,2,3,4];
    /// let permu = Permutation::from_vec(vec);
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Result<Permutation<T>, & 'static str> {
        let permu = Permutation {permu : vec};
        
        match permu.is_permu() {
            true => Ok(permu),
            false => Err("The given vector is not a permutation"),
        }
    }

    /// Initializes a Permutation with the given vector.
    /// No checking is done to the given vector, the
    /// permutation can be initialized with a vector that 
    /// is not a permutation.
    /// 
    /// # Example
    /// ```
    /// use permu_rs::permutation::Permutation;
    /// let vec : Vec<u16> = vec![0,1,2,3,4];
    /// let permu : Permutation<u16> = Permutation::from_vec_unsec(vec);
    /// ```
    pub fn from_vec_unsec(vec: Vec<T>) -> Permutation<T> {
        Permutation { permu : vec }
    }

    /// Generates a random permutation of the length given.
    ///
    /// # Panics
    /// If the length given is grater than the maximum value that `T` can hold,
    /// the method will panic.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::Permutation;
    /// let rand_permu : Permutation<u16> = Permutation::random(8);
    /// assert!(rand_permu.is_permu());
    /// assert_eq!(8, rand_permu.permu.len());
    /// ```
    pub fn random(length: usize) -> Permutation<T> {
        let mut permu: Vec<T> = Vec::with_capacity(length);
        
        let zero = T::from(0u8);
        
        let max = match T::try_from(length) {
            Ok(v) => v,
            Err(_) => panic!("Can not create a permutation longer than the max size of the its type"),
        };

        while permu.len() < length {  
            // Generate random number. n : [0, length)
            let n = rand::thread_rng().gen_range(zero, max);

            if !Self::contains(&permu, n) {
                permu.push(n);
            }
        }
        Permutation{ permu : permu }
    }
    
    /// Returns an identity vector of the length given.
    ///
    /// # Panics
    /// If the length given is grater than the maximum value that `T` can hold,
    /// the method will panic.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::Permutation;
    /// let identity : Permutation<u8> = Permutation::identity(5);
    /// assert_eq!(vec![0,1,2,3,4], identity.permu);
    /// ```
    pub fn identity(length: usize) -> Permutation<T> {
        let mut identity: Vec<T> = Vec::new();

        for i in 0..length  {
            identity.push(match T::try_from(i) {
                Ok(v) => v,
                Err(_) => panic!("Can not create a permutation longer than the max size of the its type"),
            });
        }
        Permutation { permu : identity }
    }

    /// Checks if the give `Permutation` contains an element inside.
    /// If the element is inside `Permutation` returns true.
    fn contains(permu: &Vec<T>, item: T) -> bool {
        permu.iter().any(|&x| x == item)
    }
    
    /// Checks if the vector inside `Permutation` is really a permutation.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::Permutation;
    /// let permu1 : Permutation<u8> = Permutation::from_vec_unsec(vec![0,1,2,3]);
    /// let permu2 : Permutation<u8> = Permutation::from_vec_unsec(vec![1,2,3]);
    /// let permu3 : Permutation<u8> = Permutation::from_vec_unsec(vec![0,1,4,3]);
    /// let permu4 : Permutation<u8> = Permutation::from_vec_unsec(vec![0,1,1,3]);
    ///
    /// assert!(permu1.is_permu());
    /// assert!(!permu2.is_permu()); // Not permutation
    /// assert!(!permu3.is_permu()); // Not permutation
    /// assert!(!permu4.is_permu()); // Not permutation
    /// ```
    pub fn is_permu(&self) -> bool {
        (0..self.permu.len()).all(|i| {
            // NOTE:
            // This will never panic as the boundaries of the 
            // type T will always be respected here. 
            // i : [0, permu.len] <= T.max_value()
            let elem = match T::try_from(i) {
                Ok(v) => v, 
                Err(_) => panic!("Length conversion failed"),
            };
            Self::contains(&self.permu, elem)
        })
    }
}

#[cfg(test)]
mod tests_permu {

    use crate::permutation::Permutation;
    
    #[test]
    fn generate_rand_permus() {
        for _i in 0..1000 {
            let permu : Permutation<u8> = Permutation::random(40);
            assert!(permu.is_permu());
        }
    }
}

/// Probability distribution for permutation populations.
pub struct PermuDistribution {
    distribution : Vec<Vec<usize>>,
    soften : bool,
}

/// Population of `Permutations`.
pub struct PermuPopulation<T> {
    pub population : Vec<Permutation<T>>,
    pub size : usize,
}

impl<T> PermuPopulation<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    TryInto<usize> +
    PartialEq<T> +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    Display + // NOTE : For debugging
    Debug, // NOTE : For debugging
{
    /// Returns a `PermuPopulation` created from a vector of `Permutation`.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::{Permutation, PermuPopulation};
    /// let vec = vec![Permutation::identity(5),
    ///                Permutation::random(5)];
    /// let pop = PermuPopulation::<u8>::from_vec(vec);
    /// assert_eq!(2, pop.size);
    /// ```
    pub fn from_vec(vec: Vec<Permutation<T>>) -> PermuPopulation<T> {
        let size = vec.len();
        PermuPopulation {population : vec, size : size} 
    }

    /// Returns a `PermuPopulation` of the size given with `Permutations` filled with zeros . 
    /// The permutation's length must be specified. 
    ///
    /// # Panics
    /// Internally converts `0usize` to 
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    /// // Creates a population of 10 permutations with length 20
    /// let pop : PermuPopulation<u8> = PermuPopulation::zeros(10, 20);
    /// ```
    pub fn zeros(size: usize, length: usize) -> PermuPopulation<T> {
        let zero = T::from(0u8);
        let zeros = vec![zero;length];

        let mut pop : Vec<Permutation<T>> = Vec::new(); 

        (0..size).for_each(|_| pop.push(Permutation::from_vec_unsec(zeros.clone())));

        PermuPopulation {population: pop, size : size}
    }    
    
    /// Initializes a `PermuPopulation` of random `Permutations` of the size and length given.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    /// let pop : PermuPopulation<u8> = PermuPopulation::random(10, 5);
    /// pop.population.iter().for_each(|p| assert!(p.is_permu())); // All permutations
    /// assert_eq!(pop.size, pop.population.len()); // PermuPopulation size check
    /// ```
    pub fn random(size: usize, length: usize) -> PermuPopulation<T> {
        let mut pop : Vec<Permutation<T>> = Vec::with_capacity(size);   // Initialize
        (0..size).for_each(|_| pop.push(Permutation::random(length)) ); // Generate
        PermuPopulation { population : pop, size : size}
    }

    /// Returns a probability distribution `PermuDistribution` learned from the current `PermuPopulation`.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    /// let pop = PermuPopulation::<u8>::random(10, 5);
    /// let distr = pop.learn();
    /// ```
    ///
    // NOTE: (i : positions, j : values)
    // NOTE: TO OPTIMIZE!
    pub fn learn(&self) -> PermuDistribution{ 
        let m = self.population[0].permu.len(); // Number of positions
        
        let mut distr: Vec<Vec<usize>> = vec![vec![0; m]; m]; // Init distribution matrix
        
        for i in 0..self.size {                             // For each permutation in population
            for j in 0..self.population[0].permu.len() {    // For each position in a permutation
                let e : usize = match self.population[i].permu[j].try_into() {
                    Ok(v) => v,
                    Err(_) => panic!(),
                }; 
                distr[e][j] += 1;
            }
        }
        PermuDistribution { distribution : distr , soften : false }
    }
}

/// Implementation for trait `Sampleable`.
impl<T> Population for PermuPopulation<T> where 
    T : Copy +
    From<u8> +
    TryFrom<usize> +
    TryInto<usize> +
    PartialEq<T> +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    Display + // NOTE : For debugging
    Debug, // NOTE : For debugging
{

    fn sample(&self, out: &mut PermuPopulation<T>) -> Result<(), &'static str> {
        let distribution = self.learn();
        out.population[0] = Permutation::identity(out.population[0].permu.len());
        Ok(())
    }
}

#[cfg(test)]
mod test_learn {
    use crate::permutation::PermuPopulation;
    use crate::Population;

    #[test]
    fn test() {
        let pop = PermuPopulation::<u8>::random(10, 5);
        pop.population.iter().for_each(|p| println!("{:?}", p.permu));

        let mut samples = PermuPopulation::<u8>::zeros(10, 5);

        pop.sample(&mut samples).unwrap();
        samples.population.iter().for_each(|p| println!("{:?}", p.permu));
        // panic!();
    }
}
