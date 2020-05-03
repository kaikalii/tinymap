use std::collections::{BTreeSet, HashSet};

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
    let mut group = c.benchmark_group("set_insert");

    for n in ((START / STEP)..=(END / STEP)).map(|n| n * STEP) {
        group.bench_with_input(BenchmarkId::new("array_set_50", n), &n, |b, &n| {
            b.iter_batched(
                || arrayset!(usize; 50),
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("tiny_set_20", n), &n, |b, &n| {
            b.iter_batched(
                || tinyset!(usize; 20),
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("hash_set", n), &n, |b, &n| {
            b.iter_batched(
                HashSet::new,
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("btree_set", n), &n, |b, &n| {
            b.iter_batched(
                BTreeSet::new,
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
}

pub fn insert_1_item(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_insert_1_item");

    group.bench_function("array_set_50", |b| {
        b.iter(|| {
            for i in (0..100).map(|_| random_item()) {
                let mut set = arrayset!(usize; 50);
                set.insert(i);
            }
        })
    });
    group.bench_function("tiny_set_20", |b| {
        b.iter(|| {
            for i in (0..100).map(|_| random_item()) {
                let mut set = tinyset!(usize; 20);
                set.insert(i);
            }
        })
    });
    group.bench_function("hash_set", |b| {
        b.iter(|| {
            for i in (0..100).map(|_| random_item()) {
                let mut set = HashSet::new();
                set.insert(i);
            }
        })
    });
    group.bench_function("btree_set", |b| {
        b.iter(|| {
            for i in (0..100).map(|_| random_item()) {
                let mut set = BTreeSet::new();
                set.insert(i);
            }
        })
    });
}

pub fn get(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_get");

    for n in ((START / STEP)..=(END / STEP)).map(|n| n * STEP) {
        group.bench_with_input(BenchmarkId::new("array_set_50", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = arrayset!(usize; 50);
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("tiny_set_20", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = tinyset!(usize; 20);
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("hash_set", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = HashSet::new();
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("btree_set", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = BTreeSet::new();
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.get(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
}

pub fn remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_remove");

    for n in ((START / STEP)..=(END / STEP)).map(|n| n * STEP) {
        group.bench_with_input(BenchmarkId::new("array_set_50", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = arrayset!(usize; 50);
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("tiny_set_20", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = tinyset!(usize; 20);
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("hash_set", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = HashSet::new();
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("btree_set", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut set = BTreeSet::new();
                    for i in (0..n).map(|_| random_item()) {
                        set.insert(i);
                    }
                    set
                },
                |mut set| {
                    for i in (0..n).map(|_| random_item()) {
                        set.remove(&i);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, insert, insert_1_item, get, remove);
criterion_main!(benches);
