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
//!
//! # Algorithm Descriptions
//!
//! This section seeks to give an overview of the different algorithms contained
//! in this module. See individual functions for usage guidelines.
//!
//! ## Hamming Distance Algorithm
//!
//! The hamming distance between two equal length strings is the number of
//! substitutions required to change one string into the other. Computation is
//! very simple; all that is required is to iterate through and track
//! differences. Hamming distance is implemented by [`hamming`] and
//! [`hamming_iter`]
//!
//! ## Levenshtein distance Algorithm
//!
// The funcition [levenshtein][crate::algorithms::levenshtein] implements the
// algorithm below.
//
// $$ \operatorname{lev}_{a,b}(i,j) = \begin{cases} \max(i,j) &\text{if }
// \min(i,j) = 0, \\
//    \min \begin{cases} \operatorname{lev}_{a,b}(i-1,j) + 1 \\
//         \operatorname{lev}_{a,b}(i,j-1) + 1 \\
//         \operatorname{lev}_{a,b}(i-1,j-1) + 1_{(a_i \ne b_i)}
//       \end{cases}
//    &\text{otherwise} \end{cases}$$
//
// _(erm... I can't seem to get KaTeX working. Let me know on GitHub if you can
// help!)_
//!
//! The funcition [levenshtein][crate::algorithms::levenshtein] implements the
//! following algorithm. Basically, the tool parses from top left to bottom
//! right to create a table like follows, for the classic example:
//!
//! ```text
//!      j → 0  1  2  3  4  5  6  7
//! i             str B  →
//! ↓           s  i  t  t  i  n  g
//! 0  s    [0, 1, 2, 3, 4, 5, 6, 7]
//! 1  t  k [1, 1, 2, 3, 4, 5, 6, 7]
//! 2  r  i [2, 2, 1, 2, 3, 4, 5, 6]
//! 3     t [3, 3, 2, 1, 2, 3, 4, 5]
//! 4  A  t [4, 4, 3, 2, 1, 2, 3, 4]
//! 5  ↓  e [5, 5, 4, 3, 2, 2, 3, 4]
//! 6     n [6, 6, 5, 4, 3, 3, 2, 3]
//! ```
//!
//! The outer rows (at i=0 and j=0) are just incrementing and can be filled in
//! automatically. Then, the algorithm works its way left-to-right then
//! top-to-bottom to fill in the table. Rules are:
//!
//! 1. Insertion cost is the value of the field to the left plus one
//! 2. Deletion cost is the value of the field above plus one
//! 3. Substitution cost is the value of the field left above if the two
//!    relevant characters are the same. If they are different, add one to this
//!    value.
//! 4. Take the minimum of these three values and put it in the current box.
//!
//! Eventually, the above matrix can be filled out and the final "distance" is
//! the number at the bottom right; in this case, 3.
//!
//! This library uses [an algorithm published by Sten
//! Hjelmqvist](https://www.codeproject.com/Articles/13525/Fast-memory-efficient-Levenshtein-algorithm-2)
//! and described on [the Levenshtein distance Wikipedia
//! page](https://en.wikipedia.org/wiki/Levenshtein_distance#Iterative_with_two_matrix_rows)
//! but adapted to use a single vector. Main memory usage is only that of a
//! `Vec<u32>` in the same length as the shortest string.
//!
//! ### Limited Levenshtein algorithm
//!
//! This uses the same algorithm as above, but stops matching at a desired limit
//! to avoid spending resources on obviously different strings. Use this version
//! where possible, implemented by [`levenshtein_limit`].
//!
//! ```
//! use stringmetrics::levenshtein_limit;
//! assert_eq!(levenshtein_limit("superlongstring", "", 3), 3);
//! ```
//!
//! ### Weighted Levenshtein algorithm
//!
//! Implemented by [`levenshtein_weight`] and [`levenshtein_weight_iter`], a
//! weighted Levenshtein algorithm just allows applying differing penalties for
//! insertion, deletion, and substitution. It's basically the same algorithm as
//! above, except the added values are specified (rather than always one), and
//! the initial row population is related to insertion and deletion weights.
//!
//! For example, the following code uses insertion, deletion, and substitution
//! weights of 4, 3, and 2, respectively. There is also a limit of 100:
//!
//! ```
//! use stringmetrics::{levenshtein_weight, LevWeights};
//! let weights = LevWeights::new(4, 3, 2);
//! assert_eq!(levenshtein_weight("kitten", "sitting", 100, &weights), 8);
//! ```
//! Creates the following matrix:
//!
//! ```text
//!       j → 0   1   2   3   4   5   6   7
//! i             str B  →
//! ↓             s   i   t   t   i   n   g
//! 0  s    [ 0,  4,  8, 12, 16, 20, 24, 28]
//! 1  t  k [ 3,  2,  6, 10, 14, 18, 22, 26]
//! 2  r  i [ 6,  5,  2,  6, 10, 14, 18, 22]
//! 3     t [ 9,  8,  5,  2,  6, 10, 14, 18]
//! 4  A  t [12, 11,  8,  5,  2,  6, 10, 14]
//! 5  ↓  e [15, 14, 11,  8,  5,  4,  8, 12]
//! 6     n [18, 17, 14, 11,  8,  7,  4,  8]
//! ```
//!
//! Rules are similar to above:
//!
//! 1. Insertion cost is the value of the field to the left plus insertion
//!    weight
//! 2. Deletion cost is the value of the field above plus deletion weight
//! 3. Substitution cost is the value of the field left above if the two
//!    relevant characters are the same. If they are different, add substitution
//!    weight to this value.
//! 4. Take the minimum of these three values and put it in the current box.
//!
//! The result of 8 is representative of one added letter (+4) and two
//! substitutions (+2*2). The substitution could alternatively be counted as an
//! insertion followed by a deletion but the algorithm "chooses" against it
//! since the cost would be much higher (4+3=7 when the substitution cost is
//! only 2).)
//!
//! ### Note on string comparisons
//!
//! All string-based levenshtein algorithms use bytes rather than characters by
//! default. This speeds things up significantly, and usually the difference is
//! unimportant. However, if you are working with CJK character sets or emojis,
//! you may prefer the somewhat more accurate (but slower) `chars()` usage. This
//! example presents the case well:
//!
//! ```
//! use stringmetrics::{levenshtein_limit, levenshtein_limit_iter};
//!
//! // Default implementation; these are not the "expected" values
//! // In many cases, this may be acceptable
//! assert_eq!(levenshtein_limit("鱼", "雪", 100), 2);
//! assert_eq!(levenshtein_limit("😙", "🔬", 100), 2);
//!
//! // "Expected" values by using iterator functions
//! assert_eq!(levenshtein_limit_iter("鱼".chars(), "雪".chars(), 100), 1);
//! assert_eq!(levenshtein_limit_iter("😙".chars(), "🔬".chars(), 100), 1);
//! ```
//!
//! If accurate matching on further extended unicode is required, the [unicode
//! segmentation
//! crate](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/)
//! can be used to split on the iterable `graphemes(true)`.

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
