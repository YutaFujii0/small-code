use std::fmt::{Display, Formatter, Result};
use std::error;

#[derive(Debug)]
pub struct ConvertError;

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed to convert i32 to usize.")
    }
}

impl error::Error for ConvertError {}