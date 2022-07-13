## Levenshtein algorithm

### Data types

`levenshtein()` was tested with both `u32` and `usize` for its intermediates and
return value.


```
Levenshtein u8          time:   [294.19 ns 294.34 ns 294.49 ns]
Levenshtein u16         time:   [253.20 ns 253.52 ns 253.88 ns]
Levenshtein u32         time:   [253.06 ns 253.27 ns 253.47 ns]
Levenshtein usize       time:   [251.97 ns 252.57 ns 253.31 ns]

Levenshtein Long u16    time:   [1.0078 ms 1.0094 ms 1.0117 ms]
Levenshtein Long u32    time:   [1.0066 ms 1.0072 ms 1.0079 ms]
Levenshtein Long size   time:   [1.0173 ms 1.0180 ms 1.0188 ms]

Levenshtein Limit Long u16 (hit limit)
                        time:   [1.3709 µs 1.3741 µs 1.3780 µs]
Levenshtein Limit Long u32 (hit limit)
                        time:   [1.3866 µs 1.3885 µs 1.3908 µs]
Levenshtein Limit Long usize (hit limit)
                        time:   [1.4388 µs 1.4395 µs 1.4402 µs]
```

Performance was overall quite similar. `u32` was selected to allow larger
values, but provide better memory usage than `usize`. Note the above benchmarks
were run before some significant performance improvement changes.

In the future, it may be worth changing the limit functions to accept any
integer type via generic `<U>` on limit.

### A and B swapping

Benchmarks were performed to determine if it is better to have the shorter
string on the outer loop (A) or inner loop (B). The determination was made that
it does not matter much, so the smaller string was placed inside to keep the
cache vector smaller.

## Collections

Collecting `s.chars().map(|x| *x)` appears fast when benchmarked on its own, but
this performance doesn't seem to continue with further integration. This might
be due to inaccurate benchmark relying on `.count()`.
