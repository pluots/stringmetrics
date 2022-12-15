#!/bin/sh

dtime=$(date +"%Y-%m-%d_%H%M" --utc)
describe=$(git describe --always --tags)
fname="benches/results/${describe}_${dtime}.bench"

# Print CPU information to the file
cmd="echo Benchmark from $dtime on commit $describe;"
cmd=${cmd}"rustc --version;"
cmd=${cmd}"printf '\n';"
cmd=${cmd}"echo CPU information:;"
cmd=${cmd}"lscpu | grep -E 'Architecture|Model name|Socket|Thread|CPU\(s\)|MHz';"
cmd=${cmd}"printf '\n\n\n';"
cmd=${cmd}"cargo bench --features bench $*;"

eval "$cmd" | tee "$fname"

# Run a benchmark and record the output with color
# script -q -c "$cmd cargo bench $*" "$fname"

# fix line endings
# tr -d '\r' <  "$fname" > .tmp_bench && mv .tmp_bench "$fname"
# rm .tmp_bench
