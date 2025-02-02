struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}


fn main() {
    let mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);

    let caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        ..mocha
    };

    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        caramel_macchiato.name, caramel_macchiato.price, caramel_macchiato.is_hot
    );

    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        mocha.name, mocha.price, mocha.is_hot
    );

}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
