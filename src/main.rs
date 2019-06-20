#![warn(clippy::all)]
use abr::abr::ABR;
use rayon_adaptive::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::iter::repeat_with;
use std::num::Wrapping;
use std::path::Path;
use std::time::Duration;
use time::precise_time_ns;

#[cfg(feature = "logs")]
extern crate rayon_logs as rayon;
use rayon::ThreadPoolBuilder;

const ITERATIONS: usize = 1000;

fn bench<S, F>(setup: S, function: F, size: usize, levels: Option<usize>) -> u64
where
    S: Fn(usize) -> ABR<u64, ()>,
    F: Fn(ABR<u64, ()>, Option<usize>) -> ABR<u64, ()>,
{
    let mut results: Vec<u64> = vec![];

    for _ in 0..ITERATIONS {
        let setup_result = setup(size);
        let begin = precise_time_ns();

        let _ = function(setup_result, levels); // the let is to avoid measuring tree drop in measurements

        let end = precise_time_ns();

        results.push(end - begin);
    }

    results.iter().cloned().sum::<u64>() / (ITERATIONS as u64)
}

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

fn log_results(
    data_writer: &mut BufWriter<File>,
    constant_writer: &mut BufWriter<File>,
    level: Option<usize>,
    result: Duration,
) -> Result<(), Error> {
    if let Some(l) = level {
        data_writer.write_all(format!("{} {}\n", l, result.as_nanos()).as_bytes())?;
    } else {
        constant_writer.write_all(format!("{}\n", result.as_nanos()).as_bytes())?;
    }
    Ok(())
}

// tests depending on :
// - Parallel or sequential
// - Tree size
// - Levels

fn main() -> Result<(), Error> {
    let sizes: Vec<usize> = vec![1000, 10_000, 20_000, 30_000];
    let levels: Vec<Option<usize>> = (0usize..20)
        .map(|x| if x == 0 { None } else { Some(x) })
        .collect();

    std::fs::create_dir_all("./bench_results")?;

    for size in &sizes {
        let data_file = File::create(Path::new(&format!("./bench_results/{}.data", size)))?;
        let constant_file = File::create(Path::new(&format!("./bench_results/{}.constant", size)))?;

        let mut data_writer = BufWriter::new(data_file);
        let mut constant_writer = BufWriter::new(constant_file);

        for level in &levels {
            let duration = Duration::from_nanos(bench(random_tree_data, sum_par, *size, *level));
            println!(
                "Size of {} and with level {:?} : {} µs",
                size,
                level,
                duration.as_micros()
            );

            log_results(&mut data_writer, &mut constant_writer, *level, duration)?;
        }
    }
    Ok(())
}
