use stringmetrics::spellcheck::affix::Affix;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::collections::{HashSet,BTreeSet};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use mycrate::fibonacci;

/// Load lines from a file
/// Strip the affix "/" directive
fn lines_loader() ->Vec<String> {
    let file = File::open("dictionaries/en.dic").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut v:Vec<String> = Vec::new();

    for line in lines {
        v.push(line.unwrap().split('/').next().unwrap().to_string());
    }

    v
}

pub fn hash_setup() ->HashSet<String> {
    let hs = HashSet::from_iter(lines_loader().into_iter());
    hs
}

pub fn btree_setup() ->BTreeSet<String> {
    let bt = BTreeSet::from_iter(lines_loader().into_iter());
    bt
}

pub fn btree_contains(obj:BTreeSet<String>) {
    obj.contains("Aconcagua");
    obj.contains("prefabrication");
    obj.contains("revisionism");
    obj.contains("Accenture");
    obj.contains("sweptback");
    obj.contains("tigerish");
}

pub fn hash_contains(obj:HashSet<String>) {
    obj.contains("Aconcagua");
    obj.contains("prefabrication");
    obj.contains("revisionism");
    obj.contains("Accenture");
    obj.contains("sweptback");
    obj.contains("tigerish");
}

pub fn btree_not_contains(obj:BTreeSet<String>) {
    obj.contains("aaaaaaa");
    obj.contains("bananasplit");
    obj.contains("mapmapmap");
    obj.contains("zzzzzzz");
    obj.contains("megamango");
    obj.contains("the quick brown fox jumped over hte lazy dog");
}

pub fn hash_not_contains(obj:HashSet<String>) {
    obj.contains("aaaaaaa");
    obj.contains("bananasplit");
    obj.contains("mapmapmap");
    obj.contains("zzzzzzz");
    obj.contains("megamango");
    obj.contains("the quick brown fox jumped over hte lazy dog");
}

pub fn test_iterate<T:IntoIterator<Item=String>>(obj:T) {
    let v: Vec<_> = obj.into_iter().collect();
}


pub fn bench_btree_contains(c: &mut Criterion) {
    let bt = btree_setup();
    c.bench_function("fib 20", |b| b.iter(|| btree_contains(bt)));
}

pub fn bench_btree_not_contains(c: &mut Criterion) {
    let bt = btree_setup();
    c.bench_function("fib 20", |b| b.iter(|| btree_not_contains(bt)));
}

pub fn bench_btree_iter(c: &mut Criterion) {
    let bt = btree_setup();
    c.bench_function("fib 20", |b| b.iter(|| test_iterate(bt)));
}
pub fn hash_btree_contains(c: &mut Criterion) {
    let hs = hash_setup();
    c.bench_function("fib 20", |b| b.iter(|| hash_contains(hs)));
}

pub fn hash_btree_not_contains(c: &mut Criterion) {
    let hs = hash_setup();
    c.bench_function("fib 20", |b| b.iter(|| hash_not_contains(hs)));
}

pub fn hash_btree_iter(c: &mut Criterion) {
    let hs = hash_setup();
    c.bench_function("fib 20", |b| b.iter(|| test_iterate(hs)));
}


criterion_group!(benches, bench_btree_contains,bench_btree_not_contains,bench_btree_iter,
    hash_btree_contains,hash_btree_not_contains,hash_btree_iter);
criterion_main!(benches);
