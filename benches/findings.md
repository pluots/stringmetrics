It was found that returning a usize instead of u32 did not have any significant
effect on performance.

Collecting `s.chars().map(|x| *x)` appears fast when benchmarked on its own, but
this performance doesn't seem to continue with further integration. This might
be due to inaccurate benchmark relying on `.count()`.
