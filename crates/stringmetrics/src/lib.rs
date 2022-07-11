//! # Stringmetrics library
//!
//! `Stringmetrics` is a library for applying text- and token- based comparison
//! algorithms to determine the similarity of two strings or sets. It currently
//! includes a variety of implementations of [Levenshtein
//! distance](https://en.wikipedia.org/wiki/Levenshtein_distance), [Hamming
//! distance](https://en.wikipedia.org/wiki/Hamming_distance), and [Jaccard
//! Similarity](https://en.wikipedia.org/wiki/Jaccard_index), with more string
//! metrics expected to be added in the future. It also includes helpful
//! tokenizers for things like splitting sentences into words.
//!
//! [`algorithms`] contains the basic string metrics. All of its functions are
//! re-exported here; please see [`algorithms`] for further details.
//!
//! The [`tokenizers`] module is currently sparse, but will contain various
//! common methods of splitting strings up into words for further processing.
//!
//! # Example
//!
//! ```
//! use stringmetrics::levenshtein;
//!
//! assert_eq!(levenshtein("kitten", "sitting"), 3);
//! ```

pub mod algorithms;
pub mod tokenizers;

pub use algorithms::*;
