//! # Levenshtein text distance calculations module
//!
//! This module contains functions for applying various closeness algorithms.

// use crate::iter::{find_eq_end_items, IterPairInfo};
use std::{cmp::min, convert::TryInto, fmt::Debug, iter::Skip};

use crate::iter::find_eq_end_items;

/// A struct that holds the costs of insertion, deletion, and substitution. Used
/// for levenshthein algorithms that require weight specifications.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LevWeights {
    insertion: u32,
    deletion: u32,
    substitution: u32,
}

impl LevWeights {
    pub fn new(w_ins: u32, w_del: u32, w_sub: u32) -> LevWeights {
        LevWeights {
            insertion: w_ins,
            deletion: w_del,
            substitution: w_sub,
        }
    }
}

impl Default for LevWeights {
    fn default() -> LevWeights {
        LevWeights::new(1, 1, 1)
    }
}

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
/// let a = "abcdefg";
/// let b = "mmmmmmm";
/// assert_eq!(levenshtein_limit(a, b, 3), 3);
/// ```
#[inline]
pub fn levenshtein_limit(a: &str, b: &str, limit: u32) -> u32 {
    levenshtein_limit_iter(a.bytes(), b.bytes(), limit)
}

// pub fn levenshtein_weight_iter(weights: &LevWeights)

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
/// let weights = LevWeights::new(4, 3, 2);
/// assert_eq!(levenshtein_weight("kitten", "sitting", 6, &weights), 6);
/// ```
///
/// With a more reasonable limit, we get a representative result. The 8 comes
/// from one added letter (4) and two substitutions.
///
/// ```
/// use stringmetrics::{levenshtein_weight, LevWeights};
/// let weights = LevWeights::new(4, 3, 2);
/// assert_eq!(levenshtein_weight("kitten", "sitting", 100, &weights), 8);
/// ```
#[inline]
pub fn levenshtein_weight(a: &str, b: &str, limit: u32, weights: &LevWeights) -> u32 {
    levenshtein_weight_iter(a.bytes(), b.bytes(), limit, weights)
}

/// Levenshthein distance computation on anything with [`Iterator`] with items
/// that have [`PartialEq`].
///
/// This can be used when Levenshthein distance is applicable to something other
/// than strings that has not yet been collected to a vector.
///
/// # Example
///
/// ```
/// use stringmetrics::{levenshtein_weight_iter, LevWeights};
/// let weights = LevWeights::default();
/// assert_eq!(levenshtein_weight_iter("abc".bytes(), "def".bytes(), 10, &weights), 3);
/// ```
pub fn levenshtein_limit_iter<I, T, D>(a: I, b: I, limit: u32) -> u32
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq,
{
    // Identical implementation to levenshtein_weight_iter, just avoiding
    // cost calculation overheads
    // A goal is to combine these, which may require some compiler tricks
    // Or putting the main functionality into a function with a lot of if/else
    let a_iter_base = a.into_iter();
    let b_iter_base = b.into_iter();

    let (a_len, b_len, start_same, end_same) =
        find_eq_end_items(a_iter_base.clone(), b_iter_base.clone());

    let a_len_u: u32 = (a_len - start_same - end_same)
        .try_into()
        .expect("Critical: > u32::MAX items");
    let b_len_u: u32 = (b_len - start_same - end_same)
        .try_into()
        .expect("Critical: > u32::MAX items");

    let a_wrk: Skip<D>;
    let b_wrk: Skip<D>;
    let a_len: u32;
    let b_len: u32;

    // We want the longer string in the inner loop
    // B will be the longer string from this point on
    if a_len_u > b_len_u {
        a_wrk = b_iter_base.skip(start_same);
        b_wrk = a_iter_base.skip(start_same);
        a_len = b_len_u;
        b_len = a_len_u;
    } else {
        a_wrk = a_iter_base.skip(start_same);
        b_wrk = b_iter_base.skip(start_same);
        a_len = a_len_u;
        b_len = b_len_u;
    }

    // Only check b_len because if a_len is 0, the loop won't happen
    if b_len == 0 {
        return min(a_len, limit);
    }

    if b_len - a_len >= limit {
        return limit;
    }

    let mut work_vec: Vec<u32> = (1..(b_len + 1)).collect();

    let mut tmp_res = b_len;

    for (i, a_item) in a_wrk.enumerate() {
        // Exit the loop if we are at the end
        if i as u32 >= a_len {
            break;
        }
        // Our "horizontal" iterations always start with the leftmost column,
        // which is the insertion cost (or substitution above)
        // temp_res is also our insertion cost base
        let mut sub_base = i as u32;
        tmp_res = sub_base + 1;

        // Go through and do our calculations. we need to preserve the "up left"
        // (sub_base) and "left" (tmp_res) values, the rest can be overwritten
        for (j, b_item) in b_wrk.clone().enumerate() {
            if j as u32 >= b_len {
                break;
            }

            let del_base = work_vec[j];

            // Insertion costs and deletion costs are their bases + 1
            // i.e., the value to the left or above plus 1
            // Substitution cost is equal to the up-left (sub_base) cost if equal,
            // otherwise up-left value + 1.
            if a_item != b_item {
                tmp_res = min(min(tmp_res, del_base), sub_base) + 1;
            } else {
                tmp_res = min(min(tmp_res, del_base) + 1, sub_base);
            }

            // As we shift to the right, our deletion square becomes our
            // substitution square
            sub_base = del_base;

            // Save our insertion cost for the next iteration
            work_vec[j] = tmp_res;
        }

        if tmp_res > limit {
            return limit;
        }
    }

    tmp_res
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
/// let weights = LevWeights::default();
/// assert_eq!(levenshtein_weight_iter("abc".bytes(), "def".bytes(), 10, &weights), 3);
/// ```
pub fn levenshtein_weight_iter<I, T, D>(a: I, b: I, limit: u32, weights: &LevWeights) -> u32
where
    I: IntoIterator<IntoIter = D>,
    D: DoubleEndedIterator<Item = T> + Clone,
    T: PartialEq,
{
    let a_iter_base = a.into_iter();
    let b_iter_base = b.into_iter();

    let (a_len, b_len, start_same, end_same) =
        find_eq_end_items(a_iter_base.clone(), b_iter_base.clone());

    let a_len_u: u32 = (a_len - start_same - end_same)
        .try_into()
        .expect("Critical: > u32::MAX items");
    let b_len_u: u32 = (b_len - start_same - end_same)
        .try_into()
        .expect("Critical: > u32::MAX items");

    let a_wrk: Skip<D>;
    let b_wrk: Skip<D>;
    let a_len: u32;
    let b_len: u32;

    let w_sub = weights.substitution;
    let w_ins;
    let w_del;

    // We want the longer string in the inner loop
    // B will be the longer string from this point on
    if a_len_u > b_len_u {
        a_wrk = b_iter_base.skip(start_same);
        b_wrk = a_iter_base.skip(start_same);
        a_len = b_len_u;
        b_len = a_len_u;
        w_ins = weights.deletion;
        w_del = weights.insertion;
    } else {
        a_wrk = a_iter_base.skip(start_same);
        b_wrk = b_iter_base.skip(start_same);
        a_len = a_len_u;
        b_len = b_len_u;
        w_ins = weights.insertion;
        w_del = weights.deletion;
    }

    // Only check b_len because if a_len is 0, the loop won't happen
    if b_len == 0 {
        return min(a_len * w_del, limit);
    }

    if b_len - a_len >= limit {
        return limit;
    }

    let equal_weights = w_ins == w_del && w_del == w_sub;

    let mut work_vec: Vec<u32> = (w_ins..((b_len * w_ins) + 1))
        .step_by(w_ins as usize)
        .collect();
    let mut tmp_res = b_len * w_ins;

    for (i, a_item) in a_wrk.enumerate() {
        // Reuse the casted variable as our loop exit if we are at the end
        if i as u32 >= a_len {
            break;
        }

        // Our "horizontal" iterations always start with the leftmost column,
        // which is the insertion cost (or substitution above)
        // temp_res is also our insertion cost base
        let mut sub_base = i as u32 * w_del;
        tmp_res = sub_base + w_del;

        // Go through and do our calculations. we need to preserve the "up left"
        // (sub_base) and "left" (tmp_res) values, the rest can be overwritten
        for (j, b_item) in b_wrk.clone().enumerate() {
            if j as u32 >= b_len {
                break;
            }

            let del_base = work_vec[j];

            // Insertion costs and deletion costs are their bases + 1
            // i.e., the value to the left or above plus 1
            if equal_weights {
                if a_item != b_item {
                    tmp_res = min(min(tmp_res, del_base), sub_base) + w_ins;
                } else {
                    tmp_res = min(min(tmp_res, del_base) + w_ins, sub_base);
                }
            } else if a_item != b_item {
                tmp_res = min(min(tmp_res + w_ins, del_base + w_del), sub_base + w_sub);
            } else {
                tmp_res = min(min(tmp_res + w_ins, del_base + w_del), sub_base);
            }

            // As we shift to the right, our deletion square becomes our
            // substitution square
            sub_base = del_base;

            // Save our insertion cost for the next iteration
            work_vec[j] = tmp_res;
        }

        if tmp_res > limit {
            return limit;
        }
    }

    tmp_res
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
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_equal() {
        assert_eq!(levenshtein("abcdef", "abcdef"), 0);
    }

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("abcdef", ""), 6);
        assert_eq!(levenshtein("", "abcdef"), 6);
    }

    #[test]
    fn test_levenshtein_basic() {
        assert_eq!(levenshtein("abcd", "ab"), 2);
        assert_eq!(levenshtein("ab", "abcd"), 2);
        assert_eq!(levenshtein("abcd", "ad"), 2);
        assert_eq!(levenshtein("abcd", "cd"), 2);
        assert_eq!(levenshtein("abcd", "a"), 3);
        assert_eq!(levenshtein("abcd", "c"), 3);
        assert_eq!(levenshtein("abcd", "accd"), 1);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("sitting", "kitten"), 3);
        assert_eq!(levenshtein("not", "to a"), 3);
        assert_eq!(levenshtein("to be a bee", "not to bee"), 6);
    }

    #[test]
    fn test_levenshtein_trick_skips() {
        // Try to trick the part that skips forward and backward
        assert_eq!(levenshtein("abcd", "abcd"), 0);
        assert_eq!(levenshtein("abcd", "ad"), 2);
        assert_eq!(levenshtein("abcd", "cd"), 2);
        assert_eq!(levenshtein("abcd", "a"), 3);
        assert_eq!(levenshtein("abcd", "b"), 3);
        assert_eq!(levenshtein("abcd", "c"), 3);
        assert_eq!(levenshtein("abcd", "d"), 3);
        assert_eq!(levenshtein("a", "abcd"), 3);
        assert_eq!(levenshtein("d", "abcd"), 3);
        assert_eq!(levenshtein("notate", "to ate"), 2);
        assert_eq!(levenshtein("to ate", "notate"), 2);
        assert_eq!(levenshtein("to be a", "not to"), 6);
        assert_eq!(levenshtein("not to", "to be a"), 6);
        assert_eq!(levenshtein("abccc", "accc"), 1);
    }

    #[test]
    fn test_levenshtein_limit_one_empty() {
        assert_eq!(levenshtein_limit("abcdef", "", 3), 3);
        assert_eq!(levenshtein_limit("", "abcdef", 3), 3);
        assert_eq!(levenshtein_limit("abcdef", "", 8), 6);
        assert_eq!(levenshtein_limit("", "abcdef", 8), 6);
    }

    #[test]
    fn test_levenshtein_limit() {
        // Most of this is tested via levenshtein()
        // just need to validate limits
        assert_eq!(levenshtein_limit("abcdef", "000000", 3), 3);
        assert_eq!(levenshtein_limit("ab", "0000", 3), 3);
    }

    #[test]
    fn test_levenshtein_weight_insertion() {
        let weights = LevWeights::new(10, 1, 1);
        assert_eq!(levenshtein_weight("", "a", 100, &weights), 10);
        assert_eq!(levenshtein_weight("a", "", 100, &weights), 1);
        assert_eq!(levenshtein_weight("", "ab", 100, &weights), 20);
        assert_eq!(levenshtein_weight("ab", "", 100, &weights), 2);
        assert_eq!(levenshtein_weight("ab", "abcd", 100, &weights), 20);
        assert_eq!(levenshtein_weight("kitten", "sitting", 100, &weights), 12);
    }

    #[test]
    fn test_levenshtein_weight_deletion() {
        let weights = LevWeights::new(1, 10, 1);
        assert_eq!(levenshtein_weight("", "a", 100, &weights), 1);
        assert_eq!(levenshtein_weight("a", "", 100, &weights), 10);
        assert_eq!(levenshtein_weight("", "ab", 100, &weights), 2);
        assert_eq!(levenshtein_weight("ab", "", 100, &weights), 20);
        assert_eq!(levenshtein_weight("kitten", "sitting", 100, &weights), 3);

        let weights = LevWeights::new(1, 10, 2);
        assert_eq!(levenshtein_weight("abc", "ac", 100, &weights), 10);
        assert_eq!(levenshtein_weight("abcd", "ac", 100, &weights), 20);
    }

    #[test]
    fn test_levenshtein_weight_substitution() {
        // Note that when substitution cost is high, the algorithm will prefer
        // a deletion and insertion
        let weights = LevWeights::new(10, 10, 5);
        assert_eq!(levenshtein_weight("a", "b", 100, &weights), 5);
        let weights = LevWeights::new(10, 10, 2);
        assert_eq!(levenshtein_weight("abcd", "acc", 100, &weights), 12);
        let weights = LevWeights::new(4, 3, 2);
        assert_eq!(levenshtein_weight("kitten", "sitting", 100, &weights), 8);
    }
}
