fn main() {
    let mut coffe = String::from("Macho");
    let a = &mut coffe;
    let b = a;

    println!("{a} and {b}");
}
