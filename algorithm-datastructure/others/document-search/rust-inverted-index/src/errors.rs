use std::fmt;
use std::error;

#[derive(Debug)]
pub struct NoArgumentError;

impl fmt::Display for NoArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No keyword given. Please provide one.\nExample:\n  inverted_index startup")
    }
}

impl error::Error for NoArgumentError {}
