//! The `problems`module contains permutation based problem definitions. This problems are, the
//! quadratic assignment problem (QAP), permutation flowshop scheduling problem (PFSP) and the linear ordering
//! problem (LOP). This module also includes a common definition of a problem, the `Problem`trait.
//! Finally, the `ProblemType`enum is provided in order to get the problem type from the instance's
//! name.

use std::io;
use std::io::{BufReader, BufRead};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Debug};
use std::fs::File;
use std::cmp::max;

use rand::distributions::range::SampleRange;
use std::ops::Sub;

use crate::errors::Error;
use crate::permutation::Permutation;

/// Contains all problem types defined in this crate. Implents `TryFrom<&str>` trait, so it's
/// useful t get the problem type from the instance's name.
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

/// Contains basic functions all problem's must include.
pub trait Problem {
    /// Loads an instance of a problem from a specified path.
    fn load(path: &str) -> Result<Box<Self>, Error> where Self: Sized;
    
    /// Returns the size of the instance. Solutions for the Problem must be of the same size.
    fn size(&self) -> usize;

    /// Evaluates a given solution (`Permutation`) returning it's fitness value.
    fn evaluate<T>(&self, solution: &Permutation<T>) -> Result<usize, Error>
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
    #[doc(hidden)]
    fn lines2matrix(buffer: &mut BufReader<File>, n_lines: usize, n_elems: usize) -> Result<Vec<Vec<usize>>, Error> where Self: Sized {
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
pub struct Qap {
    size: usize,
    distance: Vec<Vec<usize>>,
    flow: Vec<Vec<usize>>,
}

impl Problem for Qap {
    
    fn load(path: &str) -> Result<Box<Self>, Error> {
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

        Ok(Box::new(Qap { size, distance, flow }))
    }

    fn size(&self) -> usize {
        self.size
    }

    fn evaluate<T>(&self, solution: &Permutation<T>) -> Result<usize, Error>
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

        // Check if the solution's length matches with the size of the problem
        if solution.len() != self.size {
            return Err(Error::LengthError);
        }

        let mut fitness = 0; 
        for i in 0..self.size {
            for j in 0..self.size {

                let fact_a: usize = match solution.permu[i].try_into() {
                    Ok(n) => n,
                    Err(_) => return Err(Error::ParseError),
                };
                let fact_b: usize = match solution.permu[j].try_into() {
                    Ok(n) => n,
                    Err(_) => return Err(Error::ParseError),
                };

                let dist_ab = self.distance[i][j];
                let flow_ab = self.flow[fact_a][fact_b];

                fitness += dist_ab*flow_ab;
            }
        }
        Ok(fitness)
    }
}

/// Permutation Flowshop Scheduling Problem definition
pub struct Pfsp {
    size: usize, // Equal to number of jobs in the problem
    n_machines: usize,
    matrix: Vec<Vec<usize>>,
}

impl Problem for Pfsp {

    fn load(path: &str) -> Result<Box<Self>, Error> {
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
        Ok(Box::new(Pfsp { size: sizes[0], n_machines: sizes[1], matrix }))
    }

    fn size(&self) -> usize {
        self.size
    }

    fn evaluate<T>(&self, solution: &Permutation<T>) -> Result<usize, Error>
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

        // Check if solution length is correct
        if solution.len() != self.size {
            return Err(Error::LengthError);
        }

        let mut tft = 0;
        let mut b = vec![0;self.n_machines];  

        for (job_i, job_n) in solution.permu.iter().enumerate() {
            let mut pt = 0;
            for machine in 0..self.n_machines {

                let job: usize = match T::try_into(*job_n) {
                    Ok(n) => n,
                    Err(_) => return Err(Error::ParseError),
                };

                if job_i == 0 && machine == 0 {
                    pt = self.matrix[machine][job];
                }
                else if job_i > 0 && machine == 0 {
                    pt = b[machine] + self.matrix[machine][job];
                }
                else if job_i == 0 && machine > 0 {
                    pt = b[machine-1] + self.matrix[machine][job];
                }
                else if job_i > 0 && machine > 0 {
                    pt = max(b[machine-1], b[machine]) + self.matrix[machine][job];
                }

                b[machine] = pt;
            }
            tft += pt;
        }
        Ok(tft)
    }
}

/// Linear Ordering Problem definition 
pub struct Lop {
    size: usize,
    matrix: Vec<Vec<usize>>,
}

impl Problem for Lop {

    fn load(path: &str) -> Result<Box<Self>, Error> {
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

        Ok(Box::new(Lop {size, matrix}))
    }
    
    fn size(&self) -> usize {
        self.size
    }

    fn evaluate<T>(&self, permu: &Permutation<T>) -> Result<usize, Error>
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
        // Check if the permu's and length and instance's size are correct
        if permu.len() != self.size {
            return Err(Error::LengthError);
        }
        
        let mut fitness = 0;
        (0..self.size-1).for_each(|i| {
                (i+1..self.size).for_each(|j| {

                    let elem1 = match permu.permu[i].try_into() {
                        Ok(a) => a,
                        Err(_) => unreachable!(),
                    };
                    let elem2 = match permu.permu[j].try_into() {
                        Ok(a) => a,
                        Err(_) => unreachable!(),
                    };

                    fitness += self.matrix[elem1][elem2];
                });
            });
        Ok(fitness) 
    }
}

#[cfg(test)]
mod test {

    use crate::problems::*;
    use std::convert::TryInto;
    use crate::permutation::Permutation;

    #[test]
    fn read_lop() {
        let instance_path = "instances/LOP/N-be75eec_150.lop";
        let ptype: ProblemType = instance_path.try_into().unwrap(); 
        
        if let ProblemType::Lop = ptype {
        } else {
            panic!("The instace type is not LOP");
        }

        let permu = Permutation::<u8>::random(150);
        let lop = Lop::load(instance_path).unwrap(); 
        
        println!("permu fitness: {}", lop.evaluate(&permu).unwrap());
    }

    #[test]
    fn read_qap() {
        let instance_path = "instances/QAP/tai100a.dat";
        let ptype: ProblemType = instance_path.try_into().unwrap(); 
        
        if let ProblemType::Qap = ptype {
        } else {
            panic!("The instace type is not LOP");
        }

        let permu = Permutation::<u8>::random(100);
        let lop = Qap::load(instance_path).unwrap(); 

        println!("permu fitness: {}", lop.evaluate(&permu).unwrap());
    }

    #[test]
    fn read_pfsp() {
        let instance_path = "instances/PFSP/tai100_20_0.fsp";

        let ptype: ProblemType = instance_path.try_into().unwrap(); 
        
        if let ProblemType::Pfsp = ptype {
        } else {
            panic!("The instace type is not LOP");
        }

        let permu = Permutation::<u8>::random(100);
        let lop = Pfsp::load(instance_path).unwrap(); 

        println!("permu fitness: {}", lop.evaluate(&permu).unwrap());
    }

    #[test]
    fn test_load() {
        use crate::permutation::PermuPopulation;

        let paths = ["PFSP/tai100_20_0.fsp", 
                     "QAP/tai100a.dat",
                     "/LOP/N-be75eec_150.lop"];
        for name in paths.iter() {
            let path = format!("instances/{}", name);

            let instance: Box< dyn Problem> = match ProblemType::try_from(path.as_str()) {
                Ok(ProblemType::Qap) => Qap::load(&path).unwrap(),
                Ok(ProblemType::Pfsp) => Pfsp::load(&path).unwrap(),
                Ok(ProblemType::Lop) => Lop::load(&path).unwrap(), 
                Err(err) => panic!(err),
            };
            
            let pop = PermuPopulation::<u16>::random(100, instance.size());

            for solution in pop.population.iter() {
                let fitness = instance.evaluate(&solution).unwrap();
            }
        }
    }
}
