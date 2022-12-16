use std::mem;

/// A struct that holds
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DamerauWeights {
    pub insertion: u32,
    pub deletion: u32,
    pub substitution: u32,
    pub transposition: u32,
}

impl DamerauWeights {
    /// Create a new `DamerauWeights` object
    #[inline]
    pub const fn new(w_ins: u32, w_del: u32, w_sub: u32, w_tspn: u32) -> Self {
        Self {
            insertion: w_ins,
            deletion: w_del,
            substitution: w_sub,
            transposition: w_tspn,
        }
    }

    // Swap insertion and deletion terms
    #[inline]
    pub(crate) fn swap(&mut self) {
        mem::swap(&mut self.insertion, &mut self.deletion);
    }
}

impl Default for DamerauWeights {
    fn default() -> Self {
        Self {
            insertion: 1,
            deletion: 1,
            substitution: 1,
            transposition: 1,
        }
    }
}
