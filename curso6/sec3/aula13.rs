#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    let dog1: String = String::new();
    let dog2: String =  String::from("Max");

    println!("{}", dog1);
    println!("{}", dog2);

    let owner = format!("Hi I'm {} the owner of {}", "Mark", dog2);

    println!("{}", owner);

    println!("{}", dog1.len());



}
