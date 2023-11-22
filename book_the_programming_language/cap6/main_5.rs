#[derive(Debug)]
enum UsState {
    Labama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let mut count = 0;

    let coin = Coin::Quarter(UsState::Alaska);

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    println!("{}", count);
}
