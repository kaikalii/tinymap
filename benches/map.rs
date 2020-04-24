use std::collections::{BTreeMap, HashMap};

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::random;
use tinymap::*;

fn random_item() -> usize {
    random::<usize>() % 100
}

const START: usize = 3;
const END: usize = 50;
const STEP: usize = 3;

pub fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_insert");

    for n in ((START / STEP)..=(END / STEP)).map(|n| n * STEP) {
        group.bench_with_input(BenchmarkId::new("array_map_50", n), &n, |b, &n| {
            b.iter_batched(
                || arraymap!(usize => usize; 50),
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("tiny_map_20", n), &n, |b, &n| {
            b.iter_batched(
                || tinymap!(usize => usize; 20),
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("hash_map", n), &n, |b, &n| {
            b.iter_batched(
                HashMap::new,
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("btree_map", n), &n, |b, &n| {
            b.iter_batched(
                BTreeMap::new,
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
}

pub fn get(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_get");

    for n in ((START / STEP)..=(END / STEP)).map(|n| n * STEP) {
        group.bench_with_input(BenchmarkId::new("array_map_50", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = arraymap!(usize => usize; 50);
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("tiny_map_20", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = tinymap!(usize => usize; 20);
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("hash_map", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = HashMap::new();
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("btree_map", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = BTreeMap::new();
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
}

pub fn remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_remove");

    for n in ((START / STEP)..=(END / STEP)).map(|n| n * STEP) {
        group.bench_with_input(BenchmarkId::new("array_map_50", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = arraymap!(usize => usize; 50);
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("tiny_map_20", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = tinymap!(usize => usize; 20);
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("hash_map", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = HashMap::new();
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("btree_map", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut map = BTreeMap::new();
                    for i in (0..n).map(|_| random_item()) {
                        map.insert(i, i);
                    }
                    map
                },
                |mut map| {
                    for i in (0..n).map(|_| random_item()) {
                        map.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, insert, get, remove);
criterion_main!(benches);
