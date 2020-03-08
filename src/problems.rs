//! The `problems`module contains other modules for different permutation based problems.
// TODO: Explain the implemented problems with more detail

use std::io;
use std::io::{BufReader, BufRead};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Debug};
use std::fs::File;

use rand::distributions::range::SampleRange;
use std::ops::Sub;

use crate::errors::Error;
use crate::permutation::Permutation;

/// Contains all problem types defined in this crate.
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
    /// Loads an instance of a problem from an specified file's path.
    fn load(path: &str) -> Result<Box<Self>, Error>;

    /// Evaluates a given solution returning it's fitness value.
    fn evaluate<T>(&self, solution: &Permutation<T>) -> Result<usize, Error>
        where T :
            Copy +
            From<u8> +
            TryFrom<usize> +
            TryInto<usize> +
            // PartialEq<T> +
            Eq +
            SampleRange +
            PartialOrd +
            Sub +
            Display + // NOTE : For debugging
            Debug;

    fn lines2matrix(buffer: &mut BufReader<File>, n_lines: usize, n_elems: usize) -> Result<Vec<Vec<usize>>, Error> {
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

    fn evaluate<T>(&self, solution: &Permutation<T>) -> Result<usize, Error>
        where T :
            Copy +
            From<u8> +
            TryFrom<usize> +
            TryInto<usize> +
            // PartialEq<T> +
            Eq +
            SampleRange +
            PartialOrd +
            Sub +
            Display + // NOTE : For debugging
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

/// Linear Ordering Problem (LOP)
pub struct Lop {
    size: usize,
    pub matrix: Vec<Vec<usize>>,
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

        // Parse each line as a row in the instance matrix
        /*
        let mut matrix: Vec<Vec<usize>>= vec![]; // Init instance's matrix
        for line in reader.lines() {
            let mut row: Vec<usize> = Vec::with_capacity(size);
            for str_num in line?.split_whitespace() {
                row.push(str_num.trim().parse().unwrap());
            }
            // Check row length
            if row.len() != size {
                return Err(Error::Io(
                        io::Error::new(io::ErrorKind::InvalidData, 
                            "All rows must have the same length as the instance size")));
            }
            matrix.push(row);
        }

        // Check for matrix size, must be equal to instance's size
        if matrix.len() != size {
            return Err(Error::Io(
                    io::Error::new(io::ErrorKind::InvalidData, 
                        "Matrix length must be equal to instance size")));
        }
        */

        let matrix = Self::lines2matrix(&mut reader, size, size)?;

        Ok(Box::new(Lop {size, matrix}))
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
            Display + // NOTE : For debugging
            Debug 
    {

        // Check if the permu's and length and instance's size are correct
        if permu.len() != self.size {
            return Err(Error::LengthError);
        }
        
        let mut fitness = 0;
        (0..self.size)
            .for_each(|i| {
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
}
