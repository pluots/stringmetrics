# Changelog

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

### Changed

### Removed



## [2.1.2] - 2022-07-24

### Changed

- Changed `jaccard_set` to take a reference rather than the a copied value



## [2.1.1] - 2022-07-21

### Changed

- Improved documentation coverage



## [2.1.0] - 2022-07-21

### Added

- New function `levenshtein_limit_iter`

### Changed

- `levenshtein_limit_weight` moved to just `levenshtein_weight` to save
  redundancy
- Significantly improved algorithm for weighted levenshtein calculations

### Removed

- `levenshtein_limit_weight_slice` has been removed. Use
  `levenshtein_weight_iter` instead.


## [2.0.7] - 2022-07-14

### Changed

- Fixed musllinux builds to work for 3.7-3.10.


## [2.0.6] - 2022-07-13

<!-- ### Added -->

### Changed

- Updated algorithm used by `levenshtein` and `levenshtein_limit` for
  significantly improved performance, especially when strings start or end with
  a large number of similar characters.


## [2.0.5] - 2022-07-13

<!-- next-url -->
[Unreleased]: https://github.com/pluots/stringmetrics/compare/v2.1.2...HEAD
[2.1.2]: https://github.com/pluots/stringmetrics/compare/v2.1.1...v2.1.2
[2.1.1]: https://github.com/pluots/stringmetrics/compare/v2.1.0...v2.1.1
[2.1.0]: https://github.com/pluots/stringmetrics/compare/v2.0.7...v2.1.0
[2.0.7]: https://github.com/pluots/stringmetrics/compare/v2.0.6...v2.0.7
[2.0.6]: https://github.com/pluots/stringmetrics/compare/v2.0.5...v2.0.6
[2.0.5]: https://github.com/pluots/stringmetrics/compare/v2.0.4...v2.0.5
