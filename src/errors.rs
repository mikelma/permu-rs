use std::fmt;
use std::io;

/// Contains all errors `permu-rs` functions can return.
#[derive(Debug)]
pub enum Error {
    /// Error returned when the shape of the given vector was not the expected.
    LengthError,
    /// Error to return when a `Permutation` is not an actual permutation.
    NotPermutation,     
    /// Error to return when an incorrect `Distribution` type is given.
    IncorrectDistrType,
    /// Error to return when a parsing error occurs.
    ParseError,
    /// IO error containing a std::io::Error that is caused.
    Io(io::Error),
    /// Error to return when an incorrect problem instance is given.
    IncorrectProblemInstance,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::LengthError => write!(f, "LenghtError: Please check the shape of the given argument"),
            Error::NotPermutation => write!(f, "NotPermutation: permutation expected but no permutation vector was found"),
            Error::IncorrectDistrType => write!(f, "IncorrectDistrType: Incorrect distribution given"),
            Error::ParseError => write!(f, "ParseError: Error occurred during a parse operation"),
            Error::Io(err) => write!(f, "IO Error: {}", err),
            Error::IncorrectProblemInstance => write!(f, "Incorrect distribution type"),
        }
    }
}

// Implement io error to permu-rs error conversion
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
/*
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

/// Error type to return when a `Permutation` is not an actual permutation.
#[derive(Debug)]
pub struct NotPermutation;

impl fmt::Display for NotPermutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Permutation expected but no permutation found")
    }
}

#[derive(Debug)]
/// Error to return when an incorrect `Distribution` type is given.
pub struct IncorrectDistrType;

impl fmt::Display for IncorrectDistrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incorrect distribution given")
    }
}
*/
