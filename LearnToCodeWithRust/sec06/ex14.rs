fn main() {
    let cake = bake_cake();
    println!("I now have a {cake} cake");
}

fn bake_cake() -> String {
    String::from("Chocolate Mousse")
}
