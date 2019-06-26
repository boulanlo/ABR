#[macro_use]
extern crate criterion;

use criterion::{Bencher, Criterion, ParameterizedBenchmark};
use std::iter::repeat_with;
use std::num::Wrapping;

extern crate abr;
use abr::abr::ABR;
use abr::node::{BoxedNode, Node};

extern crate rayon_adaptive;
use rayon_adaptive::prelude::*;
use rayon_adaptive::Policy;

use rand::seq::SliceRandom;
use rand::Rng;

fn sum_iter(tree: &ABR<u64, ()>) -> Wrapping<u64> {
    tree.iter().map(|n| Wrapping(n.key)).sum()
}

fn sum_par(tree: &ABR<u64, ()>) -> Wrapping<u64> {
    tree.par_iter().map(|n| Wrapping(n.key)).sum()
}

fn sum_par_level(tree: &ABR<u64, ()>, level: usize) -> Wrapping<u64> {
    tree.par_iter().levels(level).map(|n| Wrapping(n.key)).sum()
}

fn find_depth_first(tree: &ABR<u64, ()>, depth: usize, target: u64) -> Option<&Node<u64, ()>> {
    match tree
        .par_iter()
        .cut()
        .depth_first(depth)
        .with_policy(Policy::Join(1))
        .map(|i_par| {
            match i_par
                .levels(5)
                .iterator_fold(|mut i_seq| i_seq.find(|e| e.key == target))
                .reduce(|| None, |a, b| a.or(b))
                .ok_or(())
            {
                Ok(elem) => Err(elem),
                Err(_) => Ok(()),
            }
        })
        .try_reduce(|| (), |_, _| Ok(()))
    {
        Ok(()) => None,
        Err(elem) => Some(elem),
    }
}

fn find_normal(tree: &ABR<u64, ()>, target: u64) -> Option<&BoxedNode<u64, ()>> {
    tree.par_iter()
        .levels(5)
        .iterator_fold(|mut i_seq| i_seq.find(|e| e.key == target))
        .reduce(|| None, |a, b| a.or(b))
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

fn criterion_benchmark_depth_first(c: &mut Criterion) {
    let depths: Vec<usize> = (4..6).collect();

    let size = 200_000;

    c.bench(
        "Depth first optimisation for find()",
        ParameterizedBenchmark::new(
            "With depth_first",
            move |b: &mut Bencher, depth: &usize| {
                b.iter_with_setup(
                    || {
                        let mut vec = (0..size as u64).collect::<Vec<u64>>();

                        vec.shuffle(&mut rand::thread_rng());
                        vec.into_iter().collect()
                    },
                    |tree: ABR<u64, _>| {
                        let random = rand::thread_rng().gen_range(0, size as u64);
                        assert_eq!(find_depth_first(&tree, *depth, random).unwrap().key, random);
                        tree
                    },
                )
            },
            depths,
        )
        .with_function("Without", move |b, _| {
            b.iter_with_setup(
                || {
                    let mut vec = (0..size as u64).collect::<Vec<u64>>();

                    vec.shuffle(&mut rand::thread_rng());
                    vec.into_iter().collect()
                },
                |tree: ABR<u64, _>| {
                    let random = rand::thread_rng().gen_range(0, size as u64);
                    assert_eq!(find_normal(&tree, random).unwrap().key, random);
                    tree
                },
            )
        }),
    );
}

criterion_group!(
    benches,
    //    criterion_benchmark_par_vs_iter,
    //    criterion_benchmark_levels
    criterion_benchmark_depth_first
);
criterion_main!(benches);
