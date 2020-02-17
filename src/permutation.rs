use std::convert::TryFrom;
use std::convert::TryInto;

use std::fmt;
use fmt::{Debug, Display};

use rand::Rng;

use crate::{Population, Distribution, errors::Error };
use crate::inversion::{Inversion, InversionPopulation};

/// Contains a permutation vector and methods to generate permutations.
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Permutation<T> {
    pub permu : Vec<T>,
}

impl<T> Permutation<T> where 
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
    
    /// Initializes a Permutation with the given vector. 
    ///
    /// # Errors
    /// If the given vector is not a permutation the function will return
    /// a `NotPermutation` error. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::Permutation;
    /// 
    /// let permu_ok = Permutation::<u8>::from_vec(vec![0,4,2,1,3]).unwrap();
    ///
    /// // Returns an error as the given vector is not a permutation
    /// let permu_err = Permutation::<u8>::from_vec(vec![5,4,2,1,3]); 
    /// assert!(permu_err.is_err());
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Result<Permutation<T>, Error> {
        let permu = Permutation {permu : vec};
        
        match permu.is_permu() {
            true => Ok(permu),
            false => Err(Error::NotPermutation),
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
    
    /// Returns an identity permutation of the length given.
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
        
        (0..length).for_each(|i| {
            identity.push(match T::try_from(i) {
                Ok(v) => v,
                Err(_) => panic!("Can not create a permutation longer than the max size of the its type"),
            });
        });
       
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
    
    /// Returns `Result` containing a `Inversion` based on the `Permutation`.
    ///
    /// # Error
    /// See `Inversion::from_permu` Error section.
    ///
    /// # Example
    /// ```
    /// use permu_rs::inversion::Inversion;
    /// use permu_rs::permutation::Permutation;
    ///
    /// let permu = Permutation::<u8>::from_vec(vec![3,2,1,0]).unwrap(); 
    /// let ok_inversion: Inversion<u8> = Inversion { inversion : vec![3,2,1]};
    /// let mut base: Inversion<u8> = Inversion { inversion : vec![0,0,0] };
    ///
    /// permu.to_inversion(&mut base);
    ///
    /// assert_eq!(ok_inversion, base);
    /// ```
    pub fn to_inversion(&self, out: &mut Inversion<T>) -> Result<(), Error> {
        Inversion::from_permu(&self, out)
    }

    /// Returns `Result` containing a `Permutation` based on the given `Permutation`.
    ///
    /// # Error
    /// See `Inversion::to:permu` Error section.
    ///
    /// # Example
    /// ```
    /// use permu_rs::inversion::Inversion;
    /// use permu_rs::permutation::Permutation;
    ///
    /// let inversion : Inversion<u8> = Inversion { inversion : vec![0,0,0] }; // Base Inversion
    /// let ok_permu = Permutation::<u8>::identity(4); // Expected permutation
    /// let mut permu = Permutation::<u8>::random(4); // Random permutation 
    ///
    /// Permutation::from_inversion(&inversion, &mut permu); // Fill permu based on inversion 
    ///
    /// assert_eq!(ok_permu, permu);
    /// 
    /// ```
    pub fn from_inversion(inversion: &Inversion<T>, out: &mut Permutation<T>) -> Result<(), Error> {
        Inversion::to_permu(&inversion,out)
    }
}

/// Population of `Permutations`.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct PermuPopulation<T> {
    pub population : Vec<Permutation<T>>,
    pub size : usize,
}

impl<T> PermuPopulation<T> where 
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
    /// # Example
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    ///
    /// // Creates a population of 10 permutations with length 20
    /// let pop : PermuPopulation<u8> = PermuPopulation::zeros(10, 20);
    ///
    /// println!("Zeros population:\n{}", pop);
    /// ```
    pub fn zeros(size: usize, length: usize) -> PermuPopulation<T> {
        let zero = T::from(0u8);
        let zeros = vec![zero;length];

        let mut pop : Vec<Permutation<T>> = Vec::new(); 

        (0..size).for_each(|_| pop.push(Permutation::from_vec_unsec(zeros.clone())));

        PermuPopulation {population: pop, size : size}
    }    
    /// Creates a `PermuPopulation` of identity `Permutation`s.
    /// The number of `Permutation`s in the returned `PermuPopulation` is given by
    /// `size` parameter and the length of `Permutation`s is `length`.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation as permu;
    ///
    /// let population = permu::PermuPopulation::<u8>::identity(10, 5);
    /// population.population.iter()
    ///     .for_each(|p| assert_eq!(*p, permu::Permutation::<u8>::identity(5)));
    ///
    /// println!("Identity population:\n{}", population);
    /// ```
    pub fn identity(size: usize, length: usize) -> PermuPopulation<T> {
        let mut pop : Vec<Permutation<T>> = Vec::new(); 
        (0..size).for_each(|_| pop.push(Permutation::identity(length)));

        PermuPopulation { population : pop, size : size}
        
    }
    
    /// Initializes a `PermuPopulation` of random `Permutations` of the size and length given.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    ///
    /// let pop : PermuPopulation<u8> = PermuPopulation::random(10, 5);
    /// pop.population.iter().for_each(|p| assert!(p.is_permu())); // All permutations
    ///
    /// assert_eq!(pop.size, pop.population.len()); // PermuPopulation size check
    /// ```
    pub fn random(size: usize, length: usize) -> PermuPopulation<T> {
        let mut pop : Vec<Permutation<T>> = Vec::with_capacity(size);   // Initialize
        (0..size).for_each(|_| pop.push(Permutation::random(length)) ); // Generate
        PermuPopulation { population : pop, size : size}
    }
    
    /// Fills a given `InversionPopulation` with `inversion` representations from the
    /// `PermuPopulation`. 
    /// 
    /// # Errors
    /// Returns a `LengthError` if the size of both populations are not equal.
    ///
    /// # Panics
    /// If the length of `inversion` are not the length of `Permutation`s - 1.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    /// use permu_rs::inversion::InversionPopulation;
    ///
    /// let (size, length) = (20,10);
    /// let permus = PermuPopulation::<u8>::random(size, length);
    /// let mut inv = InversionPopulation::zeros(size, length-1); // Init inv vector population
    ///
    /// permus.to_inversion(&mut inv).unwrap();
    ///
    /// println!("{}", permus);
    /// println!("{}\n", inv);
    /// ```
    pub fn to_inversion(&self, inv_pop: &mut InversionPopulation<T>) -> Result<(), Error> {
        InversionPopulation::from_permus(&self, inv_pop)?;
        Ok(()) 
    } 
}

impl<T> Population for PermuPopulation<T> where 
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
    
    /// Implementation of `learn` method for `PermuPopulation`.
    ///
    /// # Example
    /// ```
    /// use permu_rs::{Population, Distribution};
    /// use permu_rs::permutation::{PermuPopulation, Permutation};
    ///
    /// let v = vec![Permutation::<u8>::from_vec_unsec(vec![0,1,2,3]),
    ///              Permutation::<u8>::from_vec_unsec(vec![1,2,0,3])];
    /// let pop = PermuPopulation::from_vec(v); 
    /// let distr = pop.learn();
    ///
    /// let target = vec![vec![1,1,0,0],
    ///                   vec![0,1,1,0],
    ///                   vec![1,0,1,0],
    ///                   vec![0,0,0,2]];
    ///
    /// let target = Distribution::PermuDistribution(target, false);
    /// assert_eq!(target, distr);
    /// ```
    ///
    // NOTE: (i : positions, j : values)
    fn learn(&self) -> Distribution { 
        let m = self.population[0].permu.len(); // Number of positions
        
        let mut distr: Vec<Vec<usize>> = vec![vec![0; m]; m]; // Init distribution matrix

        (0..self.size).for_each(|i| {
            (0..self.population[0].permu.len()).for_each(|j| {
                let e : usize = match self.population[i].permu[j].try_into() {
                    Ok(v) => v,
                    Err(_) => panic!(),
                }; 
                distr[j][e] += 1;
            })
        });

        Distribution::PermuDistribution(distr, false)
    }
    
    /// Implementation of `sample` method for `PermuPopulation`.
    /// 
    /// # Errors
    /// Returns a `LengthError` if the length of the output population's `Permutation`s length 
    /// is not equal to its population `Permutation`'s. Returns an `IncorrectDistrType` error if
    /// the given distribution is not `PermuDistribution`.
    ///
    /// # Example
    ///
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    /// use permu_rs::{Population, Distribution};
    ///
    /// let pop = PermuPopulation::<u8>::random(1, 5); // Population to learn from
    /// let mut samples = PermuPopulation::<u8>::zeros(10, 5); // Population to fill with samples
    /// 
    /// let mut distr = pop.learn();
    ///
    /// Population::sample(&mut distr, &mut samples).unwrap();
    ///
    /// println!("{}", samples);
    /// ```
    fn sample(distr: &mut Distribution, out: &mut PermuPopulation<T>) -> Result<(), Error> {
        
        // Check if the given Distribution is correct
        let (distr, soften) = match distr {
            Distribution::PermuDistribution(d, s) => (d, s),
            _ => return Err(Error::IncorrectDistrType), 
        };

        // Check distribution and population's permus' sizes
        let length = match distr.len() == out.population[0].permu.len() {
            true => distr.len(),
            false => return Err(Error::LengthError),
        };
        
        // Check if the distribution is soften 
        if !*soften {
            // If not, soften the distribution by adding one to every element of the matrix
            *distr = distr.iter()
                .map(|row| row.iter().map(|x| x+1).collect())
                .collect();
            *soften = true;
        }
        
        // let mut used_indx = Vec::<usize>::with_capacity(length);

        (0..out.size).for_each(|out_i| {
            
            // used_indx.clear();
            let mut used_indx = Vec::<usize>::with_capacity(length);

            // let ref_permu = Permutation::<usize>::identity(length);
            let order = Permutation::<usize>::random(length);
            
            order.permu.iter().for_each(|ord| {
                let (index_f, val_f) : (Vec<usize>, Vec<usize>) = distr[*ord].iter()
                    .enumerate()
                    .filter(|(index, _)|            // Skip the values already existing in the permutation
                        used_indx.iter() 
                                .find( |&x| *x == *index )
                                .is_none())
                    .unzip();

                let max: usize = val_f.iter().sum();
                let rand: f64 = rand::thread_rng().gen_range(0.0, max as f64);

                let mut i = 0;
                let mut s = val_f[i];
                while (s as f64) < rand {
                    i += 1;
                    s += val_f[i];
                }
                let v = index_f[i];
                // Never panics, as the boundaries of T are always respected here 
                out.population[out_i].permu[*ord] = match T::try_from(v) {
                    Ok(v) => v,
                    Err(_) => panic!("Conversion error when sampling"),
                };
                used_indx.push(index_f[i]);
            }); 
        });
        Ok(())
    }        
}

impl<T> fmt::Display for PermuPopulation<T> where 
    T : Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        // For empty distibutions
        if self.size == 0 {
            return write!(f, "[]\nPermuPopulation, shape: 0,0\n");
        }

        let mut formatted = String::from("[");

        self.population.iter()
            .take(self.size -1) // Do not take the last item
            .for_each(|permu| {
                formatted.push_str(format!("{:?},\n", permu.permu).as_str());
            });

        // Now, take the last item
        formatted.push_str(format!("{:?}]", 
                                   self.population[self.size-1].permu).as_str());

        write!(f, "{}\nPermuPopulation, shape: {},{}\n", 
               formatted, self.size, self.population[0].permu.len())
    }
}
