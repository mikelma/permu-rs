//! The `problems` module contains some permutation based problem definitions. This problems are, the
//! quadratic assignment problem (QAP), permutation flowshop scheduling problem (PFSP) and the linear ordering
//! problem (LOP). 
//! Problems are intended to be used through the `ProblemInstance` enum. Finally, the `ProblemType`enum is 
//! provided in order to get the problem type from the instance's name.

use std::io;
use std::io::{BufReader, BufRead};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Debug};
use std::fs::File;
use std::cmp::max;

use rand::distributions::range::SampleRange;
use std::ops::Sub;

use crate::errors::Error;
use crate::permutation::PermuPopulation;

/// Contains all problem types defined in this crate. Implents `TryFrom<&str>` trait, so it's
/// useful to get the problem type from the instance's name.
pub enum ProblemType {
    Qap,
    Pfsp,
    Lop,
}

impl TryFrom<&str> for ProblemType {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let splitted: Vec<&str> = path.split(".").collect();
        
        // Check if there's any extension
        if splitted.len() < 2 {
            return Err(Error::Io(
                    io::Error::new(io::ErrorKind::InvalidInput, 
                        "Instance extension not found")));
        }
        
        match splitted[1] {
            "dat" => Ok(ProblemType::Qap),
            "fsp" => Ok(ProblemType::Pfsp),
            "lop" => Ok(ProblemType::Lop),
            _ => Err(Error::Io(
                    io::Error::new(io::ErrorKind::InvalidInput, 
                        format!("Wrong instance extension {}", splitted[1])))),
        }
    }
}

/// This enum contains problem definitions.  
pub enum ProblemInstance {
    /// Quadratic Assignment Problem (QAP)
    Qap(usize, Vec<Vec<usize>>, Vec<Vec<usize>>),
    /// Permutation Flowshop Scheduling Problem (PFSP) 
    Pfsp(usize, usize, Vec<Vec<usize>>),
    /// Linear Ordering Problem (LOP) 
    Lop(usize, Vec<Vec<usize>>),
}

impl ProblemInstance {
    
    /// Returns the size of the instance. All soltions must 
    /// be of the length of the problem's size.
    pub fn size(&self) -> usize {
        match self {
            ProblemInstance::Qap(n, _, _) => *n,
            ProblemInstance::Pfsp(n, _, _) => *n,
            ProblemInstance::Lop(n, _) => *n,
        } 
    }
    
    /// Loads a `ProblemInstance` from a file given as a path.
    ///
    /// # Errors
    /// Returns an `Error::Io` error if an error occurs loading the problem 
    /// instance from the given path.
    pub fn load(path: &str) -> Result<Self, Error> {
        match ProblemType::try_from(path) {
            Ok(ProblemType::Qap) => Ok(Qap::load(&path)?),
            Ok(ProblemType::Pfsp) => Ok(Pfsp::load(&path)?),
            Ok(ProblemType::Lop) => Ok(Lop::load(&path)?), 
            Err(err) => panic!(err),
        }
    }
    
    /// Evaluates each solution of a given `PermuPopulation` and stores the fitness values inside a
    /// given fitness vector.
    ///
    /// # Example
    /// ```
    /// use permu_rs::permutation::PermuPopulation;
    /// use permu_rs::problems::ProblemInstance;
    ///
    /// let paths = ["PFSP/tai100_20_0.fsp", 
    ///              "QAP/tai100a.dat",
    ///              "/LOP/N-be75eec_150.lop"];
    /// for name in paths.iter() {
    ///     let path = format!("instances/{}", name);
    ///     let instance = ProblemInstance::load(&path).unwrap();
    ///     
    ///     let pop = PermuPopulation::<u16>::random(100, instance.size());
    ///     let mut fitness = vec![0; 100];
    ///
    ///     instance.evaluate(&pop, &mut fitness).unwrap();
    /// }
    /// ```
    pub fn evaluate<T>(&self, 
            solutions: &PermuPopulation<T>, 
            fitness_vec: &mut Vec<usize>) -> Result<(), Error>
        where T :
            Copy +
            From<u8> +
            TryFrom<usize> +
            TryInto<usize> +
            Eq +
            SampleRange +
            PartialOrd +
            Sub +
            Display +
            Debug 
    {
        match self {
            ProblemInstance::Qap(_,_,_) => Qap::evaluate(self, solutions, fitness_vec),
            ProblemInstance::Pfsp(_, _,_) => Pfsp::evaluate(self, solutions, fitness_vec),
            ProblemInstance::Lop(_,_) => Lop::evaluate(self, solutions, fitness_vec),
        } 
    }
}

/// Contains basic functions all problem's must include.
#[doc(hidden)]
trait Problem {
    /// Loads an instance of a problem from a specified path.
    fn load(path: &str) -> Result<ProblemInstance, Error>;
    
    /// Evaluates a given solution (`Permutation`) returning it's fitness value.
    fn evaluate<T>(instace: &ProblemInstance, 
        solutions: &PermuPopulation<T>, 
        fitness_vec: &mut Vec<usize>) -> Result<(), Error>
        where T :
            Copy +
            From<u8> +
            TryFrom<usize> +
            TryInto<usize> +
            Eq +
            SampleRange +
            PartialOrd +
            Sub +
            Display +
            Debug;
    
    // Utility to convert a buffer into a matrix of the specified shape.
    fn lines2matrix(buffer: &mut BufReader<File>, 
        n_lines: usize, 
        n_elems: usize) -> Result<Vec<Vec<usize>>, Error> {
        
        // Init the matrix
        let mut matrix = vec![Vec::with_capacity(n_elems); n_lines];

        for i_line in 0..n_lines {
            // Read the line and split in withespaces
            let mut line = String::new();
            buffer.read_line(&mut line)?;
            let line = line.split_whitespace();

            // Parse all numbers from str to usize
            let mut count = 0;
            for str_num in line {
                matrix[i_line].push(match str_num.trim().parse() {
                    Ok(n) => n,
                    Err(_) => return Err(Error::ParseError),
                });
                count += 1;
            }

            // Check if line length is ok
            if count != n_elems {
                return Err(Error::Io(
                        io::Error::new(io::ErrorKind::InvalidData, 
                            "All rows must have the same length as the instance size")));
            }
        }
        Ok(matrix)
    }
}

/// Quadratic Assignment Problem definition.
#[doc(hidden)]
struct Qap {}

impl Problem for Qap {
    
    fn load(path: &str) -> Result<ProblemInstance, Error> {
        // Open the file
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        // Get instance's size
        let mut size_str = String::new();
        let _n = reader.read_line(&mut size_str); // Get size
        
        let size: usize = size_str.trim()
            .parse()
            .unwrap();

        let distance = Self::lines2matrix(&mut reader, size, size)?;
        let flow = Self::lines2matrix(&mut reader, size, size)?;

        Ok(ProblemInstance::Qap(size, distance, flow))
    }

    fn evaluate<T>(instace: &ProblemInstance, 
        solutions: &PermuPopulation<T>, 
        fitness_vec: &mut Vec<usize>) -> Result<(), Error>
        where T :
            Copy +
            From<u8> +
            TryFrom<usize> +
            TryInto<usize> +
            Eq +
            SampleRange +
            PartialOrd +
            Sub +
            Display +
            Debug {
        
        // Check instance type and get instace parameters
        let (size, distance, flow) = match instace {
            ProblemInstance::Qap(size, dist, flow) => (size, dist, flow),
            _ => return Err(Error::IncorrectProblemInstance),
        };

        // Check if the solution's length matches with the size of the problem
        if solutions.population[0].len() != *size {
            return Err(Error::LengthError);
        }

        for (index, solution) in solutions.population.iter().enumerate() {
            let mut fitness = 0; 
            for i in 0..*size {
                for j in 0..*size {

                    let fact_a: usize = match solution.permu[i].try_into() {
                        Ok(n) => n,
                        Err(_) => return Err(Error::ParseError),
                    };
                    let fact_b: usize = match solution.permu[j].try_into() {
                        Ok(n) => n,
                        Err(_) => return Err(Error::ParseError),
                    };

                    let dist_ab = distance[i][j];
                    let flow_ab = flow[fact_a][fact_b];

                    fitness += dist_ab*flow_ab;
                }
            }
            fitness_vec[index] = fitness;
        }
        Ok(())
    }
}

/// Permutation Flowshop Scheduling Problem definition
#[doc(hidden)]
struct Pfsp {}

impl Problem for Pfsp {

    fn load(path: &str) -> Result<ProblemInstance, Error> {
        // Open the file
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        // Read size lines from matrix
        let mut size_str = String::new();
        let _n = reader.read_line(&mut size_str); // Ignore first line
        size_str.clear();
        let _n = reader.read_line(&mut size_str); // Get size
        
        // Parse instance's sizes
        let mut splitted = size_str.split_whitespace();
        let mut count = 0;
        let mut sizes = vec![]; // n_jobs and n_machines
        while count < 2 {
            if let Some(item) = splitted.next() {
                let num: usize = match item.trim().parse() {
                    Ok(n) => n,
                    Err(_) => continue,
                };

                sizes.push(num);
                count += 1;

            } else {
                return Err(Error::Io(io::Error::new(
                            io::ErrorKind::InvalidInput, 
                            "Cannot find size inside instance file")));
            }
        }
        // Ignore a line
        let _n = reader.read_line(&mut size_str);

        // Read the matrix
        let matrix = Self::lines2matrix(&mut reader, sizes[1], sizes[0])?;
        Ok(ProblemInstance::Pfsp(sizes[0], sizes[1], matrix))
    }

    fn evaluate<T>(instace: &ProblemInstance, 
        solutions: &PermuPopulation<T>, 
        fitness_vec: &mut Vec<usize>) -> Result<(), Error>
        where T :
            Copy +
            From<u8> +
            TryFrom<usize> +
            TryInto<usize> +
            Eq +
            SampleRange +
            PartialOrd +
            Sub +
            Display +
            Debug {

        // Check instance type and get params 
        let (size, n_machines, matrix) = match instace {
            ProblemInstance::Pfsp(n, m, mat) => (n, m, mat),
            _ => return Err(Error::IncorrectProblemInstance),
        };

        // Check if solution length is correct
        if solutions.population[0].len() != *size {
            return Err(Error::LengthError);
        }

        for (index, solution) in solutions.population.iter().enumerate() {
            let mut tft = 0;
            let mut b = vec![0;*n_machines];  
            for (job_i, job_n) in solution.permu.iter().enumerate() {
                let mut pt = 0;
                for machine in 0..*n_machines {

                    let job: usize = match T::try_into(*job_n) {
                        Ok(n) => n,
                        Err(_) => return Err(Error::ParseError),
                    };

                    if job_i == 0 && machine == 0 {
                        pt = matrix[machine][job];
                    }
                    else if job_i > 0 && machine == 0 {
                        pt = b[machine] + matrix[machine][job];
                    }
                    else if job_i == 0 && machine > 0 {
                        pt = b[machine-1] + matrix[machine][job];
                    }
                    else if job_i > 0 && machine > 0 {
                        pt = max(b[machine-1], b[machine]) + matrix[machine][job];
                    }

                    b[machine] = pt;
                }
                tft += pt;
            }
            fitness_vec[index] = tft;
        }
        Ok(())
    }
}

/// Linear Ordering Problem definition 
#[doc(hidden)]
struct Lop {}

impl Problem for Lop {

    fn load(path: &str) -> Result<ProblemInstance, Error> {
        // Open the file
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        // Get instance's size
        let mut size_str = String::new();
        let _n = reader.read_line(&mut size_str); // Get size
        
        let size: usize = size_str.trim()
            .parse()
            .unwrap();

        let matrix = Self::lines2matrix(&mut reader, size, size)?;

        Ok(ProblemInstance::Lop(size, matrix))
    }

    fn evaluate<T>(instace: &ProblemInstance, 
        solutions: &PermuPopulation<T>, 
        fitness_vec: &mut Vec<usize>) -> Result<(), Error>
        where T :
            Copy +
            From<u8> +
            TryFrom<usize> +
            TryInto<usize> +
            Eq +
            SampleRange +
            PartialOrd +
            Sub +
            Display +
            Debug 
    {
        // Check instance type and get params 
        let (size, matrix) = match instace {
            ProblemInstance::Lop(n, mat) => (n, mat),
            _ => return Err(Error::IncorrectProblemInstance),
        };

        // Check if the permu's and length and instance's size are correct
        if solutions.population[0].len() != *size {
            return Err(Error::LengthError);
        }
        
        for (index, solution) in solutions.population.iter().enumerate() {
            let mut fitness = 0;
            (0..*size-1).for_each(|i| {
                    (i+1..*size).for_each(|j| {

                        let elem1 = match solution.permu[i].try_into() {
                            Ok(a) => a,
                            Err(_) => unreachable!(),
                        };
                        let elem2 = match solution.permu[j].try_into() {
                            Ok(a) => a,
                            Err(_) => unreachable!(),
                        };

                        fitness += matrix[elem1][elem2];
                    });
                });
            fitness_vec[index] = fitness;
        }
        Ok(()) 
    }
}

#[cfg(test)]
mod test {

    use crate::problems::*;
    use std::convert::TryInto;
    use crate::permutation::{PermuPopulation, Permutation};

    #[test]
    fn read_lop() {
        let instance_path = "instances/LOP/N-be75eec_150.lop";
        let ptype: ProblemType = instance_path.try_into().unwrap(); 
        
        if let ProblemType::Lop = ptype {
        } else {
            panic!("The instace type is not LOP");
        }

        let pop = PermuPopulation::<u8>::random(10, 150);
        let instance = Lop::load(instance_path).unwrap(); 
        let mut fitness = vec![0;10];

        instance.evaluate(&pop, &mut fitness).unwrap();
        
        fitness.iter()
            .for_each(|x| println!("permu fitness: {}", x));
    }

    #[test]
    fn read_qap() {
        let instance_path = "instances/QAP/tai20b.dat";
        let ptype: ProblemType = instance_path.try_into().unwrap(); 
        
        if let ProblemType::Qap = ptype {
        } else {
            panic!("The instace type is not LOP");
        }

        let instance = Qap::load(instance_path).unwrap(); 

        let mut permu = Permutation::<u8>::from_vec_unsec(
            vec![12,6,18,16,7,2,5,3,14,0,13,9,15,1,8,10,4,19,17,11]);
        let pop = PermuPopulation::<u8>::from_vec(vec![permu]);

        let mut fitness = vec![0];
        instance.evaluate(&pop, &mut fitness).unwrap();
        assert_eq!(125551590, fitness[0]);
    }

    #[test]
    fn read_pfsp() {
        let instance_path = "instances/PFSP/tai20_5_0.fsp";

        let ptype: ProblemType = instance_path.try_into().unwrap(); 
        
        if let ProblemType::Pfsp = ptype {
        } else {
            panic!("The instace type is not LOP");
        }

        let instance = Pfsp::load(instance_path).unwrap(); 

        let mut permu = Permutation::<u8>::from_vec_unsec(
            vec![11,12,0,13,14,9,10,5,2,19,18,17,7,4,3,8,1,15,6,16]);
        permu.clone().invert(&mut permu).unwrap();
        let pop = PermuPopulation::<u8>::from_vec(vec![permu]);

        let mut fitness = vec![0];
        instance.evaluate(&pop, &mut fitness).unwrap();
        assert_eq!(14033, fitness[0]);
    }

    #[test]
    fn test_load() {
        use crate::permutation::PermuPopulation;

        let paths = ["PFSP/tai100_20_0.fsp", 
                     "QAP/tai100a.dat",
                     "/LOP/N-be75eec_150.lop"];
        for name in paths.iter() {
            let path = format!("instances/{}", name);

            let instance = match ProblemType::try_from(path.as_str()) {
                Ok(ProblemType::Qap) => Qap::load(&path).unwrap(),
                Ok(ProblemType::Pfsp) => Pfsp::load(&path).unwrap(),
                Ok(ProblemType::Lop) => Lop::load(&path).unwrap(), 
                Err(err) => panic!(err),
            };
            
            let pop = PermuPopulation::<u16>::random(100, instance.size());
            let mut fitness = vec![0; 100];

            instance.evaluate(&pop, &mut fitness).unwrap();
        }
    }
}
