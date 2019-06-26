#![warn(clippy::all)]
use abr::abr::ABR;
use abr::bencher::Bencher;
use rayon_adaptive::prelude::*;
use std::io::Error;
use std::iter::repeat_with;
use std::num::Wrapping;
use std::path::Path;

#[cfg(feature = "logs")]
extern crate rayon_logs as rayon;
use rayon::ThreadPoolBuilder;

fn random_tree_data(size: usize) -> ABR<u64, ()> {
    repeat_with(rand::random).take(size).collect()
}

fn sum_par(tree: ABR<u64, ()>, level: Option<usize>) -> ABR<u64, ()> {
    let pool = ThreadPoolBuilder::new()
        .build()
        .expect("pool creation failed");

    if let Some(l) = level {
        pool.install(|| {
            tree.par_iter()
                .levels(l)
                .map(|n| Wrapping(n.key))
                .reduce(|| Wrapping(0), |a, b| a + b)
        });
    } else {
        pool.install(|| {
            tree.par_iter()
                .map(|n| Wrapping(n.key))
                .reduce(|| Wrapping(0), |a, b| a + b)
        });
    }

    tree
}

fn main() -> Result<(), Error> {
    let sizes: Vec<usize> = vec![500_000];
    let levels: Vec<Option<usize>> = (0usize..20)
        .map(|x| if x == 0 { None } else { Some(x) })
        .collect();

    let bencher = Bencher::new(Path::new("bench_results"), random_tree_data, sum_par);

    bencher.run_benchmark("Level performance comparison", sizes, levels)?;

    Ok(())
}
