//! This module includes errors used by [`stringmetrics`].

use std::fmt;

/// An error that arises when equal lengths are required but not found.
#[derive(Debug, Clone, PartialEq)]
pub struct LengthMismatchError;

impl fmt::Display for LengthMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "items of equal length are required")
    }
}
