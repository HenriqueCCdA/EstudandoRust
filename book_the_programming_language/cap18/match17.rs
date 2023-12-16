fn main() {
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("Found a string");
    }

    println!("{:?}", s);
}
