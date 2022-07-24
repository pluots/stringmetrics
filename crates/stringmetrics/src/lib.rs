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

// Strict clippy
#![warn(
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::str_to_string,
    clippy::missing_inline_in_public_items,
    clippy::exhaustive_enums,
    clippy::pattern_type_mismatch
)]
// Pedantic config
#![allow(
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    // Below items are from "restriction"
    clippy::missing_docs_in_private_items,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::exhaustive_structs,
    clippy::shadow_unrelated,
)]

pub mod algorithms;
pub mod errors;
pub mod iter;
pub mod tokenizers;

pub use algorithms::*;
