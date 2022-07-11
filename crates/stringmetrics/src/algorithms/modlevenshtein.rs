//! # Levenshtein text distance calculations module
//!
//! This module contains functions for applying various closeness algorithms.

use std::cmp::{max, min};

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

/// Basic Levenshtein distance computation
///
/// This runs the levenshtein distance algorithm with all costs equal to 1 and
/// with no limits, which is suitable for cases where an exact distance is
/// needed. Use cases are usually those where the strings are known to not be
/// "very different" (e.g., strings have similar lengths). In most cases it is
/// better to use [`levenshtein_limit`] to avoid unnecessary computation.
///
/// Behind the scenes, this wraps [`levenshtein_limit`]. For details on
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
    levenshtein_limit(a, b, u32::MAX)
}

/// Levenshtein distance computation with a limit
///
/// This will limitate the levshtein distance up to a given maximum value. The
/// usual reason for wanting to do this is to avoid unnecessary computation when
/// a match between two strings can quickly be pruned as "different".
///
/// # Example
///
/// ```
/// use stringmetrics::levenshtein_limit;
/// let a = "abcdefg";
/// let b = "mmmmmmm";
/// assert_eq!(levenshtein_limit(a, b, 3), 3);
/// ```
pub fn levenshtein_limit(a: &str, b: &str, limit: u32) -> u32 {
    // This implementation is the same as levenshtein_limit_weight_slice, but
    // without the cost multiplications (for speed).
    let a_len = a.len() as u32;
    let b_len = b.len() as u32;

    // Start with some shortcut solution optimizations
    if a_len == 0 {
        return min(b_len, limit);
    }
    if b_len == 0 {
        return min(a_len, limit);
    }

    if (max(a_len, b_len) - min(a_len, b_len)) >= limit {
        return limit;
    }

    let v_len = b_len + 1;
    let mut v_prev: Vec<u32> = (0..(v_len)).collect();
    let mut v_curr: Vec<u32> = vec![0; v_len as usize];
    let mut current_max: u32 = 0;

    // Benches show it's quicker to have these defined outside the loop
    // Than immut inside
    let mut ins_cost: u32;
    let mut del_cost: u32;
    let mut sub_cost: u32;

    for (i, a_item) in a.chars().enumerate() {
        v_curr[0] = (i + 1) as u32;
        // Fill out the rest of the row
        for (j, b_item) in b.chars().enumerate() {
            ins_cost = v_curr[j] + 1;
            del_cost = v_prev[j + 1] + 1;
            sub_cost = match a_item == b_item {
                true => v_prev[j],
                false => v_prev[j] + 1,
            };

            v_curr[j + 1] = min(min(ins_cost, del_cost), sub_cost);
        }

        current_max = *v_curr.last().unwrap();

        if current_max >= limit {
            return limit;
        }

        // Move current row to previous for the next loop
        // "Current" is always overwritten so we can just swap
        std::mem::swap(&mut v_prev, &mut v_curr);
    }

    current_max
}

/// Levenshtein distance computation with weights
///
/// Allows setting costs for inserts, deletes and substitutions. See
/// [algorithms](crate::algorithms) for details on weight computation.
///
/// Behind the scenes, this wraps [`levenshtein_limit_weight`].
///
/// # Example
///
/// In this example, an insertion weight of 4, deletion weight of 3, and
/// substitution weight of 2 are used.
///
/// ```
/// use stringmetrics::{levenshtein_weight, LevWeights};
/// let weights = LevWeights::new(4, 3, 2);
/// assert_eq!(levenshtein_weight("kitten", "sitting", &weights), 8);
/// ```
#[inline]
pub fn levenshtein_weight(a: &str, b: &str, weights: &LevWeights) -> u32 {
    levenshtein_limit_weight(a, b, u32::MAX, weights)
}

/// Levenshtein distance computations with adjustable weights and a limit
///
///
/// # Example
///
/// In this example, an insertion weight of 4, deletion weight of 3, and
/// substitution weight of 2 are used. A limit of 6 is applied, and we see that
/// we hit that limit.
///
/// ```
/// use stringmetrics::{levenshtein_limit_weight, LevWeights};
/// let weights = LevWeights::new(4, 3, 2);
/// assert_eq!(levenshtein_limit_weight("kitten", "sitting", 6, &weights), 6);
/// ```
///
/// With a more reasonable limit, we get a representative result. The 8 comes
/// from one added letter (4) and two substitutions.
///
/// ```
/// use stringmetrics::{levenshtein_limit_weight, LevWeights};
/// let weights = LevWeights::new(4, 3, 2);
/// assert_eq!(levenshtein_limit_weight("kitten", "sitting", 100, &weights), 8);
/// ```
#[inline]
pub fn levenshtein_limit_weight(a: &str, b: &str, limit: u32, weights: &LevWeights) -> u32 {
    levenshtein_limit_weight_slice(
        &a.chars().collect::<Vec<char>>(),
        &b.chars().collect::<Vec<char>>(),
        limit,
        weights,
    )
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
/// use stringmetrics::{levenshtein_limit_weight_iter, LevWeights};
/// let weights = LevWeights::default();
/// assert_eq!(levenshtein_limit_weight_iter("abc".bytes(), "def".bytes(), 10, &weights), 3);
/// ```
#[inline]
pub fn levenshtein_limit_weight_iter<I, T>(a: I, b: I, limit: u32, weights: &LevWeights) -> u32
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    // Need to collect to vectors first so we get a finite length
    let a_vec: Vec<T> = a.collect();
    let b_vec: Vec<T> = b.collect();

    levenshtein_limit_weight_slice(&a_vec, &b_vec, limit, weights)
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
pub fn levenshtein_limit_weight_slice<T: PartialEq>(
    a: &[T],
    b: &[T],
    limit: u32,
    weights: &LevWeights,
) -> u32 {
    let a_len = a.len() as u32;
    let b_len = b.len() as u32;

    let w_ins = weights.insertion;
    let w_del = weights.deletion;
    let w_sub = weights.substitution;

    // Start with some shortcut solution optimizations
    if a_len == 0 {
        return min(b_len * w_ins, limit);
    }
    if b_len == 0 {
        return min(a_len * w_del, limit);
    }

    let diff = max(a_len, b_len) - min(a_len, b_len);
    if diff >= limit {
        return limit;
    }

    // These vectors will hold the "previous" and "active" distance row, rather
    // than needing to construct the entire array. We want to keep these small
    // so a vector of u32 is preferred over usize. u16 would be even better but
    // for long text, that could be hit somewhat easily.
    let v_len = b_len + 1;
    let mut v_prev: Vec<u32> = (0..(v_len * w_ins)).step_by(w_ins as usize).collect();
    let mut v_curr: Vec<u32> = vec![0; v_len as usize];

    let mut current_max: u32 = 0;
    // i holds our "vertical" position, j our "horizontal". We fill the table
    // top to bottom. Note there is actually an offset of 1 from i to the "true"
    // array position (since we start one row down).
    for (i, a_item) in a.iter().enumerate() {
        v_curr[0] = ((i + 1) * w_del as usize) as u32;

        // Fill out the rest of the row
        for (j, b_item) in b.iter().enumerate() {
            let ins_cost = v_curr[j] + w_ins;
            let del_cost = v_prev[j + 1] + w_del;
            let sub_cost = match a_item == b_item {
                true => v_prev[j],
                false => v_prev[j] + w_sub,
            };

            v_curr[j + 1] = min(min(ins_cost, del_cost), sub_cost);
        }

        current_max = *v_curr.last().unwrap();

        if current_max >= limit {
            return limit;
        }

        // Move current row to previous for the next loop
        // "Current" is always overwritten so we can just swap
        std::mem::swap(&mut v_prev, &mut v_curr);
    }

    current_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein("", ""), 0);
    }

    #[test]
    fn test_levenshtein_equal() {
        assert_eq!(levenshtein("abcdef", "abcdef"), 0);
    }

    #[test]
    fn test_levenshtein_one_empty() {
        assert_eq!(levenshtein("abcdef", ""), 6);
        assert_eq!(levenshtein("", "abcdef"), 6);
    }

    #[test]
    fn test_levenshtein_basic() {
        assert_eq!(levenshtein("abcd", "ab"), 2);
        assert_eq!(levenshtein("abcd", "ad"), 2);
        assert_eq!(levenshtein("abcd", "cd"), 2);
        assert_eq!(levenshtein("abcd", "a"), 3);
        assert_eq!(levenshtein("abcd", "c"), 3);
        assert_eq!(levenshtein("abcd", "accd"), 1);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("to be a bee", "not to bee"), 6);
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
        assert_eq!(levenshtein_weight("", "a", &weights), 10);
        assert_eq!(levenshtein_weight("a", "", &weights), 1);
        assert_eq!(levenshtein_weight("", "ab", &weights), 20);
        assert_eq!(levenshtein_weight("ab", "", &weights), 2);
        assert_eq!(levenshtein_weight("ab", "abcd", &weights), 20);
        assert_eq!(levenshtein_weight("kitten", "sitting", &weights), 12);
    }

    #[test]
    fn test_levenshtein_weight_deletion() {
        let weights = LevWeights::new(1, 10, 1);
        assert_eq!(levenshtein_weight("", "a", &weights), 1);
        assert_eq!(levenshtein_weight("a", "", &weights), 10);
        assert_eq!(levenshtein_weight("", "ab", &weights), 2);
        assert_eq!(levenshtein_weight("ab", "", &weights), 20);
        assert_eq!(levenshtein_weight("kitten", "sitting", &weights), 3);

        let weights = LevWeights::new(1, 10, 2);
        assert_eq!(levenshtein_weight("abc", "ac", &weights), 10);
        assert_eq!(levenshtein_weight("abcd", "ac", &weights), 20);
    }

    #[test]
    fn test_levenshtein_weight_substitution() {
        // Note that when substitution cost is high, the algorithm will prefer
        // a deletion and insertion
        let weights = LevWeights::new(10, 10, 5);
        assert_eq!(levenshtein_weight("a", "b", &weights), 5);
        let weights = LevWeights::new(10, 10, 2);
        assert_eq!(levenshtein_weight("abcd", "acc", &weights), 12);
        let weights = LevWeights::new(4, 3, 2);
        assert_eq!(levenshtein_weight("kitten", "sitting", &weights), 8);
    }

    #[test]
    fn test_levenshtein_limit_weight_slice() {
        // Note that when substitution cost is high, the algorithm will prefer
        // a deletion and insertion
        let weights = LevWeights::new(1, 1, 1);
        assert_eq!(
            levenshtein_limit_weight_slice(&[1, 2, 3], &[1, 2, 2], 10, &weights),
            1
        );
        let weights = LevWeights::new(1, 5, 1);
        assert_eq!(
            levenshtein_limit_weight_slice(&[1, 2, 3], &[1, 2], 10, &weights),
            5
        );
    }
}
