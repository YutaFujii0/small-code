use std::fmt;

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error. Please check that dataset of your input file is valid.")
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub struct IterationError;

impl fmt::Display for IterationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "It have not explored every node but failed to pick next destination.")
    }
}

impl std::error::Error for IterationError {}
