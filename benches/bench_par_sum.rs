#[macro_use]
extern crate criterion;

use criterion::{Criterion, ParameterizedBenchmark};
use std::iter::repeat_with;
use std::num::Wrapping;

extern crate abr;
use abr::abr::ABR;

extern crate rayon_adaptive;
use rayon_adaptive::prelude::*;

fn sum_iter(tree: &ABR<u64, ()>) -> Wrapping<u64> {
    tree.iter()
        .map(|n| Wrapping(n.key))
        .fold(Wrapping(0), |acc, x| acc + x)
}

fn sum_par(tree: &ABR<u64, ()>) -> Wrapping<u64> {
    tree.par_iter()
        .map(|n| Wrapping(n.key))
        .reduce(|| Wrapping(0), |a, b| a + b)
}

fn criterion_benchmark(c: &mut Criterion) {
    let tree: ABR<u64, _> = repeat_with(rand::random)
        .take(200_000)
        .into_iter()
        .collect();

    c.bench(
        "Binary Search Tree (BST) key sum",
        ParameterizedBenchmark::new("Parallel", |b, i| b.iter(|| sum_par(i)), vec![tree])
            .with_function("Iterative", |b, i| b.iter(|| sum_iter(i))),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
