fn main() {
    let multiplier = 3;

    let calculation: i32 = {
        let value = 5 + 4;
        value * multiplier
    };

    println!("{calculation}")
}
