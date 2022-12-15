//! # Levenshtein text distance calculations module
//!
//! This module contains functions for applying various closeness algorithms. It is not reexporeted.

use std::iter::Skip;
// use crate::iter::{find_eq_end_items, IterPairInfo};
use crate::iter::find_eq_end_items;
use std::mem;
use std::{cmp::min, fmt::Debug};

// fn get_similar_count<I,

/// Basic Levenshtein distance computation
///
/// This runs the levenshtein distance algorithm with all costs equal to 1 and
/// with no limits, which is suitable for cases where an exact distance is
/// needed. Use cases are usually those where the strings are known to not be
/// "very different" (e.g., strings have similar lengths). In most cases it is
/// better to use [`levenshtein_limit`] to avoid unnecessary computation.
///
/// Behind the scenes, this wraps [`levenshtein_limit_iter`]. For details on
/// operation, see the [algorithms](crate::algorithms) page.
///
/// # Example
///
/// ```
/// use stringmetrics::levenshtein;
/// let a = "this is a book";
/// let b = "i am a cook";
/// assert_eq!(levenshtein(a, b), 6);
/// ```
///
/// Note that sometimes the levenshtein distance is defined as having a default
/// weight of 2 for substitutions. That isn't the case for this implementation -
/// if you need that functionality, please use [`levenshtein_weight`].
#[inline]
pub fn levenshtein(a: &str, b: &str) -> u32 {
    levenshtein_limit_iter(a.bytes(), b.bytes(), u32::MAX)
}

/// Levenshtein distance computation with a limit
///
/// This will limitate the levshtein distance up to a given maximum value. The
/// usual reason for wanting to do this is to avoid unnecessary computation when
/// a match between two strings can quickly be pruned as "different".
///
/// This function also wraps [`levenshtein_limit_iter`].
///
/// # Example
///
/// ```
/// use stringmetrics::levenshtein_limit;
///
/// let a = "abcdefg";
/// let b = "mmmmmmm";
/// assert_eq!(levenshtein_limit(a, b, 3), 3);
/// ```
#[inline]
pub fn levenshtein_limit(a: &str, b: &str, limit: u32) -> u32 {
    levenshtein_limit_iter(a.bytes(), b.bytes(), limit)
}

/// The same alrogithm as [`levenshtein_limit`] but return an `Option` to
/// indicate if the limit is exceeded
///
/// ```
/// use stringmetrics::try_levenshtein;
///
/// let a = "abcdefg";
/// let b = "mmmmmmmmm";
/// assert_eq!(try_levenshtein(a, b, 3), None);
/// ```
#[inline]
pub fn try_levenshtein(a: &str, b: &str, limit: u32) -> Option<u32> {
    try_levenshtein_iter(a.bytes(), b.bytes(), limit)
}

/// Levenshtein distance computations with adjustable weights and a limit
///
/// Allows setting costs for inserts, deletes and substitutions. See
/// [algorithms](crate::algorithms) for details on weight computation.
///
/// Behind the scenes, this wraps [`levenshtein_weight_iter`].
///
/// # Example
///
/// In this example, an insertion weight of 4, deletion weight of 3, and
/// substitution weight of 2 are used. A limit of 6 is applied, and we see that
/// we hit that limit.
///
/// ```
/// use stringmetrics::{levenshtein_weight, LevWeights};
///
/// let weights = LevWeights::new(4, 3, 2);
/// assert_eq!(levenshtein_weight("kitten", "sitting", 6, &weights), 6);
/// ```
///
/// With a more reasonable limit, we get a representative result. The 8 comes
/// from one added letter (4) and two substitutions.
///
/// ```
/// use stringmetrics::{levenshtein_weight, LevWeights};
///
/// let weights = LevWeights::new(4, 3, 2);
/// assert_eq!(levenshtein_weight("kitten", "sitting", 100, &weights), 8);
/// ```
#[inline]
pub fn levenshtein_weight(a: &str, b: &str, limit: u32, weights: &LevWeights) -> u32 {
    levenshtein_weight_iter(a.bytes(), b.bytes(), limit, weights)
}

/// The same algorithm as [`levenshtein_weight`] but return an `Option` to
/// indicate if the limit is exceeded
#[inline]
pub fn try_levenshtein_weight(a: &str, b: &str, limit: u32, weights: &LevWeights) -> Option<u32> {
    try_levenshtein_weight_iter(a.bytes(), b.bytes(), limit, weights)
}

/// Levenshthein distance computation on anything with [`Iterator`] with items
/// that have [`PartialEq`].
///
/// This can be used when Levenshthein distance is applicable to something other
/// than strings. It wraps [`try_levenshtein_iter`].
///
/// # Example
///
/// ```
/// use stringmetrics::levenshtein_limit_iter;
///
/// assert_eq!(levenshtein_limit_iter("abc".bytes(), "def".bytes(), 10), 3);
/// assert_eq!(levenshtein_limit_iter("abc".bytes(), "def".bytes(), 10), 3);
/// ```
#[inline]
pub fn levenshtein_limit_iter<I, T, D>(a: I, b: I, limit: u32) -> u32
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq,
{
    try_levenshtein_iter(a, b, limit).unwrap_or(limit)
}

/// The same algorithm as [`levenshtein_limit_iter`] but return an `Option` to
/// indicate if the limit is exceeded
///
/// Returns `Some(u32)` if a distance is found, `None` if a limit is hit
///
/// # Example
///
/// ```
/// use stringmetrics::try_levenshtein_iter;
///
/// assert_eq!(try_levenshtein_iter("abc".bytes(), "abd".bytes(), 10), Some(1));
/// assert_eq!(try_levenshtein_iter("abcdef".bytes(), "wxyz".bytes(), 3), None);
/// ```
#[inline]
pub fn try_levenshtein_iter<I, T, D>(a: I, b: I, limit: u32) -> Option<u32>
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq,
{
    // Identical implementation to levenshtein_weight_iter, just avoiding
    let state = LevState::new(a.into_iter(), b.into_iter());
    let LevState {
        a_iter,
        b_iter,
        a_diff_len: a_len,
        b_diff_len: b_len,
    } = state;

    // Only check b_len because if a_len is 0, the loop won't happen
    if b_len == 0 {
        return Some(min(a_len, limit));
    }

    if b_len - a_len > limit {
        return None;
    }
    if b_len - a_len >= limit {
        return Some(limit);
    }

    let mut work_vec: Vec<u32> = (1..=b_len).collect();
    let mut tmp_res = b_len;

    for (i, a_item) in a_iter.enumerate().take_while(|&(i, _)| i < a_len as usize) {
        // Our "horizontal" iterations always start with the leftmost column,
        // which is the insertion cost (or substitution above)
        // temp_res is also our insertion cost base
        let mut sub_base = i as u32;
        tmp_res = sub_base + 1;

        // Go through and do our calculations. we need to preserve the "up left"
        // (sub_base) and "left" (tmp_res) values, the rest can be overwritten
        for (j, b_item) in b_iter
            .clone()
            .enumerate()
            .take_while(|&(j, _)| j < b_len as usize)
        {
            let del_base = work_vec[j];

            // Insertion costs and deletion costs are their bases + 1
            // i.e., the value to the left or above plus 1
            // Substitution cost is equal to the up-left (sub_base) cost if equal,
            // otherwise up-left value + 1.
            if a_item == b_item {
                tmp_res = min(min(tmp_res, del_base) + 1, sub_base);
            } else {
                tmp_res = min(min(tmp_res, del_base), sub_base) + 1;
            }

            // As we shift to the right, our deletion square becomes our
            // substitution square
            sub_base = del_base;

            // Save our insertion cost for the next iteration
            work_vec[j] = tmp_res;
        }

        if tmp_res > limit {
            return None;
        }
    }

    Some(tmp_res)
}

/// Weighted Levenshthein distance computation on anything with [`Iterator`]
/// with items that have [`PartialEq`].
///
/// This can be used when Levenshthein distance is applicable to something other
/// than strings that has not yet been collected to a vector.
///
/// # Example
///
/// ```
/// use stringmetrics::{levenshtein_weight_iter, LevWeights};
///
/// let weights = LevWeights::default();
/// assert_eq!(levenshtein_weight_iter("abc".bytes(), "def".bytes(), 10, &weights), 3);
/// ```
#[inline]
pub fn levenshtein_weight_iter<I, T, D>(a: I, b: I, limit: u32, weights: &LevWeights) -> u32
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq,
{
    try_levenshtein_weight_iter(a, b, limit, weights).unwrap_or(limit)
}

/// The same algorithm as [`levenshtein_weight_iter`] but return an `Option` to
/// indicate if the limit is exceeded
#[inline]
pub fn try_levenshtein_weight_iter<I, T, D>(
    a: I,
    b: I,
    limit: u32,
    weights: &LevWeights,
) -> Option<u32>
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq,
{
    let mut weights = weights.clone();
    let state = LevState::new_weights(a.into_iter(), b.into_iter(), &mut weights);
    let LevState {
        a_iter,
        b_iter,
        a_diff_len: a_len,
        b_diff_len: b_len,
    } = state;
    let LevWeights {
        insertion: w_ins,
        deletion: w_del,
        substitution: w_sub,
    } = weights;

    // Only check b_len because if a_len is 0, the loop won't happen
    if b_len == 0 {
        return Some(min(a_len * w_del, limit));
    }

    if b_len - a_len > limit {
        return None;
    }

    if b_len - a_len >= limit {
        return Some(limit);
    }

    let equal_weights = w_ins == w_del && w_del == w_sub;

    let mut work_vec: Vec<u32> = (w_ins..=(b_len * w_ins)).step_by(w_ins as usize).collect();
    let mut tmp_res = b_len * w_ins;

    for (i, a_item) in a_iter.enumerate().take_while(|&(i, _)| i < a_len as usize) {
        // Our "horizontal" iterations always start with the leftmost column,
        // which is the insertion cost (or substitution above)
        // temp_res is also our insertion cost base
        let mut sub_base = i as u32 * w_del;
        tmp_res = sub_base + w_del;

        // Go through and do our calculations. we need to preserve the "up left"
        // (sub_base) and "left" (tmp_res) values, the rest can be overwritten
        for (j, b_item) in b_iter
            .clone()
            .enumerate()
            .take_while(|&(j, _)| j < b_len as usize)
        {
            let del_base = work_vec[j];

            // Insertion costs and deletion costs are their bases + 1
            // i.e., the value to the left or above plus 1
            if equal_weights {
                if a_item == b_item {
                    tmp_res = min(min(tmp_res, del_base) + w_ins, sub_base);
                } else {
                    tmp_res = min(min(tmp_res, del_base), sub_base) + w_ins;
                }
            } else if a_item == b_item {
                tmp_res = min(min(tmp_res + w_ins, del_base + w_del), sub_base);
            } else {
                tmp_res = min(min(tmp_res + w_ins, del_base + w_del), sub_base + w_sub);
            }

            // As we shift to the right, our deletion square becomes our
            // substitution square
            sub_base = del_base;

            // Save our insertion cost for the next iteration
            work_vec[j] = tmp_res;
        }

        if tmp_res > limit {
            return None;
        }
    }

    Some(tmp_res)
}

/// A struct that holds the costs of insertion, deletion, and substitution. Used
/// for levenshthein algorithms that require weight specifications.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LevWeights {
    insertion: u32,
    deletion: u32,
    substitution: u32,
}

impl LevWeights {
    #[inline]
    pub const fn new(w_ins: u32, w_del: u32, w_sub: u32) -> Self {
        Self {
            insertion: w_ins,
            deletion: w_del,
            substitution: w_sub,
        }
    }

    // Swap insertion and deletion terms
    #[inline]
    fn swap(&mut self) {
        mem::swap(&mut self.insertion, &mut self.deletion);
    }
}

impl Default for LevWeights {
    #[inline]
    fn default() -> Self {
        Self::new(1, 1, 1)
    }
}

#[derive(Debug)]
struct LevState<D: DoubleEndedIterator> {
    a_iter: Skip<D>,
    b_iter: Skip<D>,
    a_diff_len: u32,
    b_diff_len: u32,
}

impl<D: DoubleEndedIterator<Item = T> + Clone, T: PartialEq> LevState<D> {
    fn new_inner(a_iter: D, b_iter: D) -> Self {
        let iter_info = find_eq_end_items(a_iter.clone(), b_iter.clone());
        let skip = iter_info.start_same as usize;
        Self {
            a_iter: a_iter.skip(skip),
            b_iter: b_iter.skip(skip),
            a_diff_len: iter_info.a_diff_len(),
            b_diff_len: iter_info.b_diff_len(),
        }
    }

    // Create a new state from
    #[inline]
    fn new(a_iter: D, b_iter: D) -> Self {
        let mut ret = Self::new_inner(a_iter, b_iter);
        if ret.should_swap() {
            ret.swap_inner();
        }
        ret
    }

    /// Create a new structure and swap weights if needed
    #[inline]
    fn new_weights(a_iter: D, b_iter: D, weights: &mut LevWeights) -> Self {
        let mut ret = Self::new_inner(a_iter, b_iter);
        if ret.should_swap() {
            ret.swap_inner();
            weights.swap();
        }
        ret
    }

    /// We want the longer string in B so it's in the inner loop
    #[inline]
    const fn should_swap(&self) -> bool {
        self.a_diff_len > self.b_diff_len
    }

    #[inline]
    fn swap_inner(&mut self) {
        mem::swap(&mut self.a_iter, &mut self.b_iter);
        mem::swap(&mut self.a_diff_len, &mut self.b_diff_len);
    }
}

/// Levenshtein distance computations with adjustable weights and a limit, for
/// any slice.
///
/// This function implements calculation of the [levenshtein
/// distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two
/// strings, with specified costs for insertion, deletion, and substitution, and
/// a limit. The other non-iterator functions in this module simply wrap it, and
/// it's generally easier to use any of those (e.g. [`levenshtein_limit`])
/// unless you need all the functionality that this has to offer.
///
/// Note that this algorithm does not apply any sort of per-character weights,
/// as some implementations may allow for. Instead, it assumes that all
/// substitutions have a cost of 0 if the characters are equal, and the
/// specified weight if the characters are not equal.
///
/// See [algorithms](crate::algorithms) for a detailed description of the
/// algorithm in use.

#[cfg(test)]
mod tests;
