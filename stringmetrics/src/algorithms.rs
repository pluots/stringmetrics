//! # Stringmetric Algorithms
//!
//! This module includes the various implementations for Levenshthein and
//! Hamming distance, as well as the Jaccard index. See these modules for
//! in-depth explanation of how the algorithms work, or the function docs for
//! usage information
//!
//! All relevant functions can be directly imported from `stringmetrics`, no
//! need to access them nested modules (see the example below).
//!
//! ## Example
//!
//! ```
//! use stringmetrics::levenshtein;
//! let a = "this is a book";
//! let b = "i am a cook";
//! assert_eq!(levenshtein(a, b), 6);
//! ```

mod modhamming;
// mod damerau;
mod modjaccard;
mod modlevenshtein;

pub use self::modhamming::{hamming, hamming_iter};
// pub use self::damerau::damerau_levenshtein;
pub use self::modjaccard::{jaccard, jaccard_set};
pub use self::modlevenshtein::{
    levenshtein, levenshtein_limit, levenshtein_limit_iter, levenshtein_weight,
    levenshtein_weight_iter, LevWeights,
};
