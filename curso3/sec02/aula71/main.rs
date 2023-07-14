mod coin;

use coin::Coin;


fn main() {

    let mut coin = Coin::new(1);
    println!("Value of coin is {}", coin.get_value());

    coin.set_value(2);
    println!("Value of coin is {}", coin.get_value());

}
