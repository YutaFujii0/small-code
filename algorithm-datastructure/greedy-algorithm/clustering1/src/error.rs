use std::fmt;
use std::error;

#[derive(Debug)]
pub struct ArgumentError;

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured")
    }
}

impl error::Error for ArgumentError {}
