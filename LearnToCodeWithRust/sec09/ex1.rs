fn main() {
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,
    };

    println!("My {} this morning cost {}. It is {} that it was hot", mocha.name, mocha.price, mocha.is_hot);

    let favorite_coffer = mocha.name;

    println!("{}", mocha.price
);

}
