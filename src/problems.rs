//! The `problems`module contains other modules for different permutation based problems.
// TODO: Explain the implemented problems with more detail

use std::io;
use std::convert::TryFrom;
use crate::errors::Error;

enum ProblemType {
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
            "qap" => Ok(ProblemType::Qap),
            "fsp" => Ok(ProblemType::Pfsp),
            "lop" => Ok(ProblemType::Lop),
            _ => Err(Error::Io(
                    io::Error::new(io::ErrorKind::InvalidInput, 
                        format!("Wrong instance extension {}", splitted[1])))),
        }
    }
}

/// Instances for different permutation based problems.
/// Every instance contains its dimension and one or more matrix.
#[derive(Debug)]
pub enum ProblemInstance {
    Qap(usize, Vec<Vec<usize>>, Vec<Vec<usize>>),
    Pfsp(usize, Vec<Vec<usize>>),
    Lop(usize, Vec<Vec<usize>>),
}

impl ProblemInstance {
    /// Returns a `str` with the name of the instance.
    pub fn name(&self) -> &str {
        match *self {
            ProblemInstance::Qap(_, _, _) => "QAP",
            ProblemInstance::Pfsp(_, _) => "PFSP",
            ProblemInstance::Lop(_, _) => "PFSP",
        } 
    }  

    fn load_instance(path: &str) -> Result<ProblemInstance, Error> {
        // Determine problem's type
        // Split path's name and extension
        match ProblemType::try_from(path)? {
            ProblemType::Qap => unimplemented!(),
            ProblemType::Pfsp => pfsp::load(path),
            ProblemType::Lop => lop::load(path),
        }
    }
}

/// Module for the Quadratic Assignment Problem (QAP)
pub mod qap {

}

/// Module for the Permutation Flowshop Problem (Pfsp)
pub mod pfsp {

    use crate::problems::ProblemInstance;
    use crate::errors::Error;
    use crate::problems::lop;

    pub fn load(path: &str) -> Result<ProblemInstance, Error> {
        if let ProblemInstance::Lop(n, matrix) = lop::load(path)? {
            return Ok(ProblemInstance::Pfsp(n, matrix));
        }
        unreachable!();
    }
}

/// Module for the Linear Ordering Problem (LOP)
mod lop {
    use crate::problems::ProblemInstance;
    use crate::errors::Error;
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    use std::io;

    pub fn load(path: &str) -> Result<ProblemInstance, Error> {
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

        Ok(ProblemInstance::Lop(size, matrix))
    }
}

#[cfg(test)]
mod test {

    use crate::problems::*;

    #[test]
    fn read_lop() {
        if let ProblemInstance::Lop(n, matrix) = ProblemInstance::load_instance("instances/LOP/N-be75eec_150.lop").unwrap() {
            assert_eq!(150, n); 
            assert_eq!(150, matrix.len()); 
            assert_eq!(150, matrix[0].len()); 
        } else {
            panic!("The instace type is not LOP");
        }
    }

    #[test]
    fn read_pfsp() {
        if let ProblemInstance::Lop(n, matrix) = ProblemInstance::load_instance("instances/PFSP/").unwrap() {
            assert_eq!(150, n); 
            assert_eq!(150, matrix.len()); 
            assert_eq!(150, matrix[0].len()); 
        } else {
            panic!("The instace type is not LOP");
        }
    }
}
