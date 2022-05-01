use std::fmt;

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error. Please check that dataset of your input file is valid.")
    }
}

impl std::error::Error for ParseError {}
