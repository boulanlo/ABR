use abr::abr::ABR;
use rayon_adaptive::prelude::*;
use rayon_adaptive::Policy;
use std::iter::repeat_with;
use time::precise_time_ns;
#[cfg(feature = "logs")]
extern crate rayon_logs as rayon;

use rayon::ThreadPoolBuilder;

fn main() {
    /*let tree: ABR<_, _> = vec![5, 3, 7, 1, 4, 2, 6].into_iter().collect();
    let results: u32 = tree
        .par_iter()
        .map(|n| n.key)
        //.with_policy(Policy::Sequential)
        .sum();
    //let results: u32 = tree.par_iter().to_sequential().map(|n| n.key).sum();
    println!("{}", results);
    */

    let input: Vec<u64> = repeat_with(rand::random).take(1000_000).collect();
    let tree: ABR<_, _> = input.iter().collect();
    let start = precise_time_ns();
    let sorted_input: Vec<_> = tree.iter().map(|n| n.key.clone()).collect();
    let end = precise_time_ns();
    println!("sequential took {} ns", end - start);
    assert!(sorted_input.windows(2).all(|w| w[0] < w[1]));

    let pool = ThreadPoolBuilder::new()
        .build()
        .expect("pool creation failed");

    let start = precise_time_ns();
    let s: u64 = pool.install(|| tree.par_iter().map(|n| *n.key).reduce(|| 0, |a, b| a + b));
    assert!(s > 0);
    //let sorted_input:Vec<u64> = pool.install(|| (0..10_000_000u64).into_par_iter().collect());
    let end = precise_time_ns();
    println!("parallel took {} ns", end - start);
    assert!(sorted_input.windows(2).all(|w| w[0] < w[1]));
}
