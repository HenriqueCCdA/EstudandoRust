#[derive(Debug)]
struct DeliSandwitch {}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(13.14));
    println!("{}", identity("hello"));
    println!("{}", identity(String::from("hello")));
    println!("{}", identity(true));
    println!("{:?}", identity(DeliSandwitch{}));
}

fn identity<T>(value: T) -> T{
    value
}
