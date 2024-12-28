fn main() {
    let is_handsome = true;
    let is_silly = false;

    println!("Handsome: {is_handsome}. Silly: {is_silly}");

    let age:i32 = 21;
    let is_young = age < 35;
    println!("{is_young}");
    println!("{} {}", age.is_positive(), age.is_negative() );
}
