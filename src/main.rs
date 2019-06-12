#![warn(clippy::all)]
use abr::abr::ABR;

fn main() {
    let mut a = ABR::new();
    println!("{:?}", a);
    a.insert("y", "y");
    a.insert("w", "w");
    a.insert("z", "z");
    a.insert("x", "x");
    println!("{:?}", a);
}
