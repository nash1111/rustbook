fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

fn main() {
    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("hello", "world");
    println!("{} {}", t1.0, t1.1);
    println!("{} {}", t2.0, t2.1);
}