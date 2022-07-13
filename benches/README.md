The directory /benches/results contains the outputs of `cargo bench`. This gives
a simple baseline for comparison when changes are made.

This should only be run at committed hashes, preferably those that will not be
squashed. Save a benchmark with the following:

```bsh
./benches/logbench.sh
```
