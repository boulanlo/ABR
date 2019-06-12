use abr::abr::ABR;

fn main() {
    let mut a = ABR::new();
    a.insert(6, "a");
    a.insert(5, "b");
    a.insert(4, "c");
    a.insert(10, "d");
    a.insert(7, "e");
    a.insert(8, "f");
    a.insert(15, "g");
    a.to_dot("examples/remove.dot");
    a.to_dot("examples/remove2.dot");
}
