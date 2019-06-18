use abr::abr::ABR;
use std::iter::repeat_with;
use std::num::Wrapping;
use time::precise_time_ns;
#[cfg(feature = "logs")]
extern crate rayon_logs as rayon;

use rayon::ThreadPoolBuilder;

extern crate rayon_adaptive;
use rayon_adaptive::prelude::*;

fn perfect_tree(level: usize, acc: &mut Vec<u64>, last: u64) {
    if level != 0 {
        let current_power = 2u64.pow(level as u32 - 1);
        let (small, big) = (last - current_power, last + current_power);
        acc.push(small);
        acc.push(big);

        perfect_tree(level - 1, acc, small);
        perfect_tree(level - 1, acc, big);
    }
}

fn create_perfect_tree_data(size: usize) -> Vec<u64> {
    let power = 2u64.pow(size as u32);
    let mut v: Vec<u64> = vec![power];
    perfect_tree(size as usize, &mut v, power);
    v
}

fn main() {
    //let input: Vec<u64> = repeat_with(rand::random).take(1_000_000).collect();
    let input: Vec<u64> = create_perfect_tree_data(24);
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
