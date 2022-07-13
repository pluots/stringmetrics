#!/bin/sh

dtime=$(date +"%Y-%m-%d %H:%MZ" --utc)
hash=$(git rev-parse --short HEAD)
fname="benches/results/$dtime $hash.bench"

# Print CPU information to the file
cmd="echo Benchmark from $dtime on commit $hash;"
cmd=${cmd}"printf '\n'; "
cmd=${cmd}"echo CPU information:; "
cmd=${cmd}"lscpu | grep -E 'Architecture|Model name|Socket|Thread|CPU\(s\)|MHz'; "
cmd=${cmd}"printf '\n\n\n';"
cmd=${cmd}"cargo bench $*;"

eval "$cmd" | tee "$fname"

# Run a benchmark and record the output with color
# script -q -c "$cmd cargo bench $*" "$fname"

# fix line endings
# tr -d '\r' <  "$fname" > .tmp_bench && mv .tmp_bench "$fname"
# rm .tmp_bench
