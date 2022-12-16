mod implementation;
pub use implementation::*;
use std::mem;

#[inline]
pub fn osa_distance(a: &str, b: &str) -> u32 {
    try_osa_iter(a.bytes(), b.bytes(), u32::MAX).unwrap_or(u32::MAX)
}

#[inline]
pub fn osa_limit(a: &str, b: &str, limit: u32) -> u32 {
    try_osa_iter(a.bytes(), b.bytes(), limit).unwrap_or(limit)
}

#[inline]
pub fn try_osa(a: &str, b: &str, limit: u32) -> Option<u32> {
    try_osa_iter(a.bytes(), b.bytes(), limit)
}

#[cfg(test)]
mod tests;
