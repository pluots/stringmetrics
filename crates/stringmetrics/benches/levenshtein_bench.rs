use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::cmp::min;
use stringmetrics::{levenshtein, levenshtein_limit};

pub fn bench_lev(c: &mut Criterion) {
    c.bench_function("Base Levenshtein", |b| {
        b.iter(|| levenshtein(black_box("the fat cat"), black_box("a mean rat")))
    });
    c.bench_function("Quick Levenshtein", |b| {
        b.iter(|| levenshtein_quick(black_box("the fat cat"), black_box("a mean rat")))
    });
    c.bench_function("Quick Levenshtein usize", |b| {
        b.iter(|| levenshtein_quick_size(black_box("the fat cat"), black_box("a mean rat")))
    });

    c.bench_function("Base Levenshtein limit", |b| {
        b.iter(|| {
            levenshtein_limit(
                black_box("the fat cat"),
                black_box("a mean rat"),
                black_box(40),
            )
        })
    });
    c.bench_function("Quick Levenshtein limit", |b| {
        b.iter(|| {
            levenshtein_quick_limit(
                black_box("the fat cat"),
                black_box("a mean rat"),
                black_box(40),
            )
        })
    });
}

pub fn bench_lev_empty(c: &mut Criterion) {
    c.bench_function("Base Levenshtein Empty", |b| {
        b.iter(|| levenshtein(black_box("the fat cat"), black_box("")))
    });

    c.bench_function("Quick Levenshtein Empty", |b| {
        b.iter(|| levenshtein_quick(black_box("the fat cat"), black_box("")))
    });
}

#[inline]
fn levenshtein_quick(a: &str, b: &str) -> u32 {
    let a_len = a.len() as u32;
    let b_len = b.len() as u32;

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let v_len = b_len + 1;
    let mut v_prev: Vec<u32> = (0..(v_len)).collect();
    let mut v_curr: Vec<u32> = vec![0; v_len as usize];

    for (i, a_item) in a.chars().enumerate() {
        v_curr[0] = (i + 1) as u32;
        // Fill out the rest of the row
        for (j, b_item) in b.chars().enumerate() {
            let ins_cost = v_curr[j] + 1;
            let del_cost = v_prev[j + 1] + 1;
            let sub_cost = match a_item == b_item {
                true => v_prev[j],
                false => v_prev[j] + 1,
            };

            v_curr[j + 1] = min(min(ins_cost, del_cost), sub_cost);
        }

        // current_max = *v_curr.last().unwrap();

        // Move current row to previous for the next loop
        // "Current" is always overwritten so we can just swap
        std::mem::swap(&mut v_prev, &mut v_curr);
    }

    *v_prev.last().unwrap()
}

fn levenshtein_quick_limit(a: &str, b: &str, limit: u32) -> u32 {
    let a_len = a.len() as u32;
    let b_len = b.len() as u32;

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let v_len = b_len + 1;
    let mut v_prev: Vec<u32> = (0..(v_len)).collect();
    let mut v_curr: Vec<u32> = vec![0; v_len as usize];
    let mut current_max: u32 = 0;

    for (i, a_item) in a.chars().enumerate() {
        v_curr[0] = (i + 1) as u32;
        // Fill out the rest of the row
        for (j, b_item) in b.chars().enumerate() {
            let ins_cost = v_curr[j] + 1;
            let del_cost = v_prev[j + 1] + 1;
            let sub_cost = match a_item == b_item {
                true => v_prev[j],
                false => v_prev[j] + 1,
            };

            v_curr[j + 1] = min(min(ins_cost, del_cost), sub_cost);
        }

        current_max = *v_curr.last().unwrap();

        if current_max >= limit {
            return limit;
        }

        // Move current row to previous for the next loop
        // "Current" is always overwritten so we can just swap
        std::mem::swap(&mut v_prev, &mut v_curr);
    }

    current_max
}

#[inline]
fn levenshtein_quick_size(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();

    // Start with some shortcut solution optimizations
    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    // These vectors will hold the "previous" and "active" distance row, rather
    // than needing to construct the entire array. We want to keep these small
    // so a vector of u32 is preferred over usize. u16 would be even better but
    // for long text, that could be hit somewhat easily.
    let v_len = b_len + 1;
    let mut v_prev: Vec<usize> = (0..(v_len)).collect();
    let mut v_curr: Vec<usize> = vec![0; v_len as usize];

    // i holds our "vertical" position, j our "horizontal". We fill the table
    // top to bottom. Note there is actually an offset of 1 from i to the "true"
    // array position (since we start one row down).
    for (i, a_item) in a.chars().enumerate() {
        v_curr[0] = i + 1;
        // Fill out the rest of the row
        for (j, b_item) in b.chars().enumerate() {
            let ins_cost = v_curr[j] + 1;
            let del_cost = v_prev[j + 1] + 1;
            let sub_cost = match a_item == b_item {
                true => v_prev[j],
                false => v_prev[j] + 1,
            };

            v_curr[j + 1] = min(min(ins_cost, del_cost), sub_cost);
        }

        // current_max = *v_curr.last().unwrap();

        // Move current row to previous for the next loop
        // "Current" is always overwritten so we can just swap
        std::mem::swap(&mut v_prev, &mut v_curr);
    }

    *v_prev.last().unwrap()
}

criterion_group!(levenshtein_bench, bench_lev, bench_lev_empty,);
criterion_main!(levenshtein_bench);
