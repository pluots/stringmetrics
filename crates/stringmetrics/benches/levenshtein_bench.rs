use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::cmp::min;
use stringmetrics::{levenshtein, levenshtein_limit, levenshtein_limit_weight, LevWeights};

const STR_A: &str = "the fat cat";
const STR_B: &str = "a mean bat";

const STR_A_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing \
    elit. Nunc non dictum elit. Curabitur vitae dapibus dolor, id consequat \
    velit. Proin a sem orci. Aenean at auctor enim, ut imperdiet metus. Donec \
    quis nisl congue, sagittis leo id, consequat est. Sed vitae pulvinar ante. \
    Donec efficitur tellus in nisi congue malesuada. Etiam hendrerit quis \
    metus a facilisis. Pellentesque lobortis rhoncus mauris, at aliquam diam \
    luctus at. In dapibus bibendum nisl. Pellentesque habitant morbi tristique \
    senectus et netus et malesuada fames ac turpis egestas. Sed commodo, urna \
    id consectetur condimentum, ex ex finibus ipsum, at mollis leo urna mattis \
    lacus. Sed egestas, nunc eu rhoncus posuere, velit libero pharetra massa, nec
    pharetra nisi nisi in est. Suspendisse a libero ipsum.";
const STR_B_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing \
    elit. Suspendisse pharetra dapibus eros ut malesuada. Nunc quis nunc odio. \
    Nunc elementum malesuada vulputate. Donec ac ex id purus sodales porttitor \
    nec et magna. Etiam tortor nunc, vulputate vel molestie et, viverra nec \
    orci. Curabitur sagittis nisl id elit congue, et lacinia erat maximus. In \
    non pharetra arcu. Praesent vitae viverra lorem. Phasellus sodales luctus \
    ullamcorper. Maecenas lorem eros, condimentum vel nisi id, sagittis pretium \
    quam. Ut non congue nulla. Donec laoreet viverra tellus, eu placerat sapien \
    malesuada id. Maecenas facilisis lectus diam, quis sodales enim scelerisque \
    ut. Vestibulum ut ex in sem placerat ultrices. Mauris felis leo, fermentum \
    id suscipit eget, sagittis non mauris. Donec ut interdum risus.";

pub fn bench_lev(c: &mut Criterion) {
    let weights = LevWeights::default();
    c.bench_function("Base Levenshtein", |b| {
        b.iter(|| levenshtein(black_box(STR_A), black_box(STR_B)))
    });
    c.bench_function("Quick Levenshtein", |b| {
        b.iter(|| levenshtein_quick(black_box(STR_A), black_box(STR_B)))
    });
    c.bench_function("Base Levenshtein limit", |b| {
        b.iter(|| levenshtein_limit(black_box(STR_A), black_box(STR_B), black_box(40)))
    });
    c.bench_function("Quick Levenshtein limit", |b| {
        b.iter(|| levenshtein_quick_limit(black_box(STR_A), black_box(STR_B), black_box(40)))
    });
    c.bench_function("Levenshtein Weights", |b| {
        b.iter(|| levenshtein_limit_weight(black_box(STR_A), black_box(STR_B), 100, &weights))
    });
}

pub fn bench_lev_empty(c: &mut Criterion) {
    let weights = LevWeights::default();

    c.bench_function("Base Levenshtein Empty", |b| {
        b.iter(|| levenshtein(black_box(STR_A), black_box("")))
    });
    c.bench_function("Quick Levenshtein Empty", |b| {
        b.iter(|| levenshtein_quick(black_box(STR_A), black_box("")))
    });
    c.bench_function("Levenshtein Weights Empty", |b| {
        b.iter(|| {
            levenshtein_limit_weight(black_box(STR_A_LONG), black_box(STR_B_LONG), 100, &weights)
        })
    });
}

pub fn bench_lev_long(c: &mut Criterion) {
    let weights = LevWeights::default();

    c.bench_function("Base Levenshtein Long", |b| {
        b.iter(|| levenshtein(black_box(STR_A_LONG), black_box(STR_B_LONG)))
    });
    c.bench_function("Limited Levenshtein Long", |b| {
        b.iter(|| levenshtein_limit(black_box(STR_A_LONG), black_box(STR_B_LONG), 100))
    });
    c.bench_function("Levenshtein Weights Long", |b| {
        b.iter(|| {
            levenshtein_limit_weight(black_box(STR_A_LONG), black_box(STR_B_LONG), 100, &weights)
        })
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

criterion_group!(
    levenshtein_bench,
    bench_lev,
    bench_lev_empty,
    bench_lev_long
);
criterion_main!(levenshtein_bench);
