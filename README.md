# Stringmetrics

This is a Rust library for approximate string matching that implements simple
algorithms such has Hamming distance, Levenshtein distance, Jaccard similarity,
and more, as well as a competent spellchecker that handles Hunspell
dictionaries.

This package comes with a library for programatic use, as well as a command line
interface. The library is usable via WASM.

Crate info:
[https://crates.io/crates/stringmetrics](https://crates.io/crates/stringmetrics)

Crate docs:
[https://docs.rs/stringmetrics/](https://docs.rs/stringmetrics/).

Crate source:
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

### Performance

In general, this program has been shown to be quite fast. Benchmarks show a
result of about 40 ns per word:

```bash
Spellcheck: compile dictionary
                        time:   [121.98 ms 122.56 ms 123.16 ms]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Spellcheck: 1 correct word
                        time:   [38.071 ns 38.339 ns 38.714 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

Spellcheck: 1 incorrect word
                        time:   [51.343 ns 51.776 ns 52.282 ns]
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe

Spellcheck: 15 correct words
                        time:   [567.37 ns 574.87 ns 582.95 ns]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

Spellcheck: 15 incorrect words
                        time:   [787.35 ns 838.96 ns 909.99 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

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
