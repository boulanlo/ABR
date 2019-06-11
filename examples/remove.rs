use abr::ABR;

fn main() {
    let mut a:ABR<_, _> = (1..10).collect();
    a.to_dot("examples/remove1.dot");
    assert_eq!(a.remove(&9), Some(()));
    a.to_dot("examples/remove2.dot");
}
