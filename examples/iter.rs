use abr::abr::ABR;

fn main() {
    let tree : ABR<u32, _> = vec![5, 3, 7, 1, 4, 2, 6].into_iter().collect();
    let a : Vec<u32> = tree.iter().map(|n| n.key).collect();
    println!("{:?}", a);
}
