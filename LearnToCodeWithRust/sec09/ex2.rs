fn main() {
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = true;


    println!("My {} this morning cost {}. It is {} that it was hot", beverage.name, beverage.price, beverage.is_hot);

}
