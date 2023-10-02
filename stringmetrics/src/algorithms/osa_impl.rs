mod implementation;
pub use implementation::*;

use crate::DamerauWeights;

#[inline]
pub fn osa_distance(a: &str, b: &str) -> u32 {
    // try_osa_iter(a.bytes(), b.bytes(), u32::MAX).unwrap_or(u32::MAX)
    try_osa_weight_iter(a.bytes(), b.bytes(), u32::MAX, &DamerauWeights::default())
        .unwrap_or(u32::MAX)
}

#[inline]
pub fn osa_limit(a: &str, b: &str, limit: u32) -> u32 {
    try_osa_iter(a.bytes(), b.bytes(), limit).unwrap_or(limit)
}

#[inline]
pub fn osa_limit_iter<I, T, D>(a: I, b: I, limit: u32) -> u32
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq + Clone,
{
    try_osa_iter(a, b, limit).unwrap_or(limit)
}

#[inline]
pub fn try_osa(a: &str, b: &str, limit: u32) -> Option<u32> {
    try_osa_iter(a.bytes(), b.bytes(), limit)
}

#[inline]
pub fn osa_weight(a: &str, b: &str, limit: u32, weights: &DamerauWeights) -> u32 {
    try_osa_weight_iter(a.bytes(), b.bytes(), limit, weights).unwrap_or(limit)
}
#[inline]
pub fn try_osa_weight(a: &str, b: &str, limit: u32, weights: &DamerauWeights) -> Option<u32> {
    try_osa_weight_iter(a.bytes(), b.bytes(), limit, weights)
}

#[inline]
pub fn osa_weight_iter<I, T, D>(a: I, b: I, limit: u32, weights: &DamerauWeights) -> u32
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq + Clone,
{
    try_osa_weight_iter(a, b, limit, weights).unwrap_or(limit)
}

#[cfg(test)]
mod tests;
