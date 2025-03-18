fn make_tuple<T, U>(firts: T, second: U) -> (T, U) {
    (firts, second)
}

fn main() {
    make_tuple("hello", 5);
    make_tuple(5, 13);
    make_tuple(true, 3.14);
    make_tuple(true, false);
}
