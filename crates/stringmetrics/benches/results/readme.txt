These files keep track of benchmarks at different points to check whether
performance improves or degrades.


This should only be run at committed hashes, preferably those that will not be
squashed. Run with the following:

```bench
script -q -c "cargo bench" benches/results/$(git rev-parse HEAD).bench
```
