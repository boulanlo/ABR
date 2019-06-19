#[macro_use]
extern crate criterion;

use criterion::{Bencher, Criterion, ParameterizedBenchmark};
use std::iter::repeat_with;
use std::num::Wrapping;

extern crate abr;
use abr::abr::ABR;

extern crate rayon_adaptive;
use rayon_adaptive::prelude::*;

fn sum_iter(tree: &ABR<u64, ()>) -> Wrapping<u64> {
    tree.iter().map(|n| Wrapping(n.key)).sum()
}

fn sum_par(tree: &ABR<u64, ()>) -> Wrapping<u64> {
    tree.par_iter().map(|n| Wrapping(n.key)).sum()
}

fn sum_par_level(tree: &ABR<u64, ()>, level: usize) -> Wrapping<u64> {
    tree.par_iter().levels(level).map(|n| Wrapping(n.key)).sum()
}

fn criterion_benchmark_par_vs_iter(c: &mut Criterion) {
    let sizes = vec![1_000, 10_000, 50_000, 100_000];

    c.bench(
        "BST Iterative vs Parallel",
        ParameterizedBenchmark::new(
            "BST Iterative sum",
            |b, input_size| {
                b.iter_with_setup(
                    || repeat_with(rand::random).take(*input_size).collect(),
                    |tree: ABR<u64, _>| {
                        sum_iter(&tree);
                        tree
                    },
                )
            },
            sizes,
        )
        .with_function("BST Parallel sum", |b, input_size| {
            b.iter_with_setup(
                || repeat_with(rand::random).take(*input_size).collect(),
                |tree: ABR<u64, _>| {
                    sum_par(&tree);
                    tree
                },
            )
        }),
    );
}

fn criterion_benchmark_levels(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "BST Parallel with level limitation",
        move |b: &mut Bencher, input: &usize| {
            b.iter_with_setup(
                || repeat_with(rand::random).take(100_000).collect(),
                |tree: ABR<u64, _>| {
                    sum_par_level(&tree, *input);
                    tree
                },
            );
        },
        0usize..21,
    );
}

criterion_group!(
    benches,
    criterion_benchmark_par_vs_iter,
    criterion_benchmark_levels
);
criterion_main!(benches);
