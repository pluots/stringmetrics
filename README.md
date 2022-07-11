# Stringmetrics

This is a Rust library for approximate string matching that implements simple
algorithms such has Hamming distance, Levenshtein distance, Jaccard similarity,
and more.

Crate info:
[https://crates.io/crates/stringmetrics](https://crates.io/crates/stringmetrics)

Crate docs:
[https://docs.rs/stringmetrics/](https://docs.rs/stringmetrics/).

Crate source:
[https://github.com/pluots/stringmetrics](https://github.com/pluots/stringmetrics)

Note that spellcheck features have been moved to
[https://crates.io/crates/zspell](https://crates.io/crates/zspell)


## Stringmetric Algorithms

One of the main purposes of this library is to provide a variety of string
metric functions. These include a few Levenshtein implementations (including
limit/max, weighted, and generic), Jaccard index, and a Hamming implementation.
These are all found in the `algorithms` module. Quick example:

```rs
// Basic levenshtein distance
use stringmetrics::levenshtein;

assert_eq!(levenshtein("kitten", "sitting"), 3);
```

```rs
// Levenshtein distance with a limit to save computation time
use stringmetrics::levenshtein_limit;

assert_eq!(levenshtein_limit("a very long string", "short!", 4), 4);
```

```rs
// Basic hamming distance
use stringmetrics::hamming;

let a = "abcdefg";
let b = "aaadefa";
assert_eq!(hamming(a, b), 3);
```

```rs
// Set custom weights
use stringmetrics::{levenshtein_weight, LevWeights};

// This struct holds insertion, deletion, and substitution costs
let weights = LevWeights::new(4, 3, 2);
assert_eq!(levenshtein_weight("kitten", "sitting", &weights), 8);
```


See [the documentation](https://docs.rs/stringmetrics/) for more details.


## License

See the LICENSE file for license information. The provided license does allow
for proprietary use and adaptation; that being said, I kindly suggest that if
you come up with an improvement, you submit a pull request and help us all out
:)
