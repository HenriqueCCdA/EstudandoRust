fn main() {
    let evaluation: bool = true;

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("{value}");
}
