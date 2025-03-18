fn main() {
    let oranges = String::from("Oranges");
    print_my_value(&oranges);
    println!("{oranges} is still valid");
}

fn print_my_value(value: &String) {
    println!("You value is {value}")

}
