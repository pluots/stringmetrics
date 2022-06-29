# Stringmetrics

This is a Rust library for approximate string matching that implements simple
algorithms such has Hamming distance, Levenshtein distance, Jaccard similarity,
and more, as well as a competent spellchecker that handles Hunspell
dictionaries.

Get the create info here:
[https://crates.io/crates/stringmetrics](https://crates.io/crates/stringmetrics)
and see the docs here
[https://docs.rs/stringmetrics/](https://docs.rs/stringmetrics/).

Source:
[https://github.com/pluots/stringmetrics-rust](https://github.com/pluots/stringmetrics-rust)

## Stringmetric Algorithms

One of the main purposes of this library is to provide a variety of string
metric functions. These include a few Levenshtein implementations (including
limit/max, weighted, and generic), Jaccard index, and a Hamming implementation.
These are all found in the `algorithms` module.


## Spellcheck

This is a spellchecker written completely in Rust. While maintaining
compatibility with the venerable Hunspell dictionary format, it does not rely on
Hunspell or any other underlying checker.

Spellcheck functionality is found in the `spellcheck` module.

### Functionality

NOTE: The spellcheck portion of this project is still under development.
Completed and future planned support include:

- [x] Basic prefix/sufix dictionary files
- [ ] Forbidden word handling
- [ ]
- [ ] Morphological/Phonetic handling

### Design Decisions

A lot of spellchecking revolves just seeing if a word exists in a very large
list. Two logical data structure choices are  `std::collections::BTreeSet` and
`std::collections::HashSet` - `HashSet` blew `BTreeSet` out of the water on a
table with 50k rows, so we went with that.

```bash
# "Contains" is a test for words that are in the list. "Not Contains" is a test for
BTree Contains          time:   [2.0070 us 2.0119 us 2.0173 us]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

BTree Not Contains      time:   [2.2838 us 2.2922 us 2.3017 us]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

BTree Iter              time:   [274.62 us 276.07 us 277.75 us]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

Hash Contains           time:   [482.35 ns 516.38 ns 560.42 ns]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) high mild
  11 (11.00%) high severe

Hash Not Contains       time:   [378.75 ns 380.30 ns 382.17 ns]
Found 17 outliers among 100 measurements (17.00%)
  7 (7.00%) high mild
  10 (10.00%) high severe

Hash Iter               time:   [176.80 us 177.37 us 178.22 us]
Found 19 outliers among 100 measurements (19.00%)
  8 (8.00%) high mild
  11 (11.00%) high severe
```

## License

See the LICENSE file for license information. The provided license does allow
for proprietary use and adaptation; that being said, I kindly suggest that if
you come up with an improvement, you submit a pull request and help us all out
:)

### Dictionary data license

The dictionaries provided in this repository for testing purposed have been
obtained under license. These files have been sourced from here:
[https://github.com/wooorm/dictionaries](https://github.com/wooorm/dictionaries)

These dictionaries are licensed under various licenses, different from that of
this project. Please see the applicable `.license` file withing the
`dictionaries/` directory.

**Note: this project was previously named "textdistance". Please make sure to
update all references.**
