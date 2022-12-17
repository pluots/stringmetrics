use crate::iter::find_eq_end_items;
use std::iter::Skip;
use std::mem;

///
pub trait WeightsSwap {
    fn swap(&mut self);
}

/// A struct that holds the costs of insertion, deletion, and substitution. Used
/// for levenshthein algorithms that require weight specifications.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LevWeights {
    pub insertion: u32,
    pub deletion: u32,
    pub substitution: u32,
}

impl LevWeights {
    /// Create a new `LevWeights` object
    #[inline]
    pub const fn new(w_ins: u32, w_del: u32, w_sub: u32) -> Self {
        Self {
            insertion: w_ins,
            deletion: w_del,
            substitution: w_sub,
        }
    }
}
impl WeightsSwap for LevWeights {
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

/// Representation of a string for lev parsing after stipping start & end
#[derive(Debug)]
pub struct LevState<D: DoubleEndedIterator> {
    pub a_iter: Skip<D>,
    pub b_iter: Skip<D>,
    /// Lengths after trimming
    pub a_len: u32,
    pub b_len: u32,
}

impl<D: DoubleEndedIterator<Item = T> + Clone, T: PartialEq> LevState<D> {
    fn new_inner(a_iter: D, b_iter: D) -> Self {
        let iter_info = find_eq_end_items(a_iter.clone(), b_iter.clone());
        let skip = iter_info.start_same as usize;
        Self {
            a_iter: a_iter.skip(skip),
            b_iter: b_iter.skip(skip),
            a_len: iter_info.a_diff_len(),
            b_len: iter_info.b_diff_len(),
        }
    }

    // Create a new state from
    #[inline]
    pub fn new(a_iter: D, b_iter: D) -> Self {
        let mut ret = Self::new_inner(a_iter, b_iter);
        if ret.should_swap() {
            ret.swap_inner();
        }
        ret
    }

    /// Create a new structure and swap weights if needed
    #[inline]
    pub fn new_weights<W: WeightsSwap>(a_iter: D, b_iter: D, weights: &mut W) -> Self {
        let mut ret = Self::new_inner(a_iter, b_iter);
        if ret.should_swap() {
            ret.swap_inner();
            weights.swap();
        }
        ret
    }

    /// We want the longer string in B so it's in the inner loop
    #[inline]
    pub const fn should_swap(&self) -> bool {
        self.a_len > self.b_len
    }

    #[inline]
    pub fn swap_inner(&mut self) {
        mem::swap(&mut self.a_iter, &mut self.b_iter);
        mem::swap(&mut self.a_len, &mut self.b_len);
    }
}
