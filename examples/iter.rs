use abr::abr::ABR;
use rayon_adaptive::prelude::*;
use rayon_adaptive::Policy;
use std::iter::repeat_with;
use std::num::Wrapping;
use time::precise_time_ns;
#[cfg(feature = "logs")]
extern crate rayon_logs as rayon;

use rayon::ThreadPoolBuilder;

fn main() {
    /*
    let tree: ABR<_, _> = vec![5, 7, 1, 6, 4, 2, 3, 8, 10, 9].into_iter().collect();

    //tree.to_dot("examples/debug.dot");

    let results: Vec<u32> = tree
        .iter()
        .map(|n| n.key)
        //.with_policy(Policy::Sequential)
        .collect();
    //let results: u32 = tree.par_iter().to_sequential().map(|n| n.key).sum();
    println!("{:?}", results);
     */

    let input: Vec<u64> = repeat_with(rand::random).take(1_000_000).collect();
    let tree: ABR<_, _> = input.iter().collect();
    //tree.to_dot("examples/debug.dot");

    let start = precise_time_ns();
    let sum_iter: Wrapping<u64> = tree
        .iter()
        .map(|n| Wrapping(*n.key))
        .fold(Wrapping(0), |acc, x| acc + x);

    let end = precise_time_ns();
    println!("seq: {} ns", end - start);
    //assert!(sorted_input.windows(2).all(|w| w[0] < w[1]));

    let pool = ThreadPoolBuilder::new()
        .build()
        .expect("pool creation failed");

    let start = precise_time_ns();
    let sum_par: Wrapping<u64> = pool.install(|| {
        tree.par_iter()
            .map(|n| Wrapping(*n.key))
            .reduce(|| Wrapping(0), |a, b| a + b)
    });

    let end = precise_time_ns();
    assert!(sum_iter.0 == sum_par.0);

    println!("par: {} ns", end - start);
    //assert!(sorted_input.windows(2).all(|w| w[0] < w[1]));
}
