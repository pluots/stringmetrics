# Stringmetrics

This is a Rust library for approximate string matching that implements simple
algorithms such has Hamming distance, Levenshtein distance, Jaccard similarity,
and more.

In addition to the basic levenshtein algorithm, it includes a weighted algorithm
that allows specifying costs of insertions, deletions, and substitutions, as
well as a maximum limit for optimization.

Get the create info here:
[https://crates.io/crates/stringmetrics](https://crates.io/crates/stringmetrics)
and see the docs here
[https://docs.rs/stringmetrics/](https://docs.rs/stringmetrics/).

Source:
[https://github.com/pluots/stringmetrics-rust](https://github.com/pluots/stringmetrics-rust)


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
