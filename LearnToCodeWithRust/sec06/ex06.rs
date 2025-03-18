fn main() {
    let person: String = String::from("Boris");
    drop(person);

    println!("{person}")
}
