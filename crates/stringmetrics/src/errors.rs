use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct LengthMismatchError;

impl fmt::Display for LengthMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "items of equal length are required")
    }
}
