fn main() {
    let number = 13;

    // match number {
    //     2 | 4 | 6 | 8 => println!("{number} is even"),
    //     1 | 3 | 5 => println!("{number} is odd"),
    //     _ => println!("Unknow for now"),
    // }
    match number {
        value if value % 2 == 0 => println!("{value} is even"),
        x if x % 2 != 0 => println!("{x} is odd"),
        _ => unreachable!(),
    }
}
