fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
}

fn identity<T>(value: T) -> T{
    value
}
