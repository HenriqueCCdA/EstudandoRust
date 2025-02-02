struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}


fn main() {
    let coffee = make_coffee(String::from("Latte"), 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        coffee.name, coffee.price, coffee.is_hot
    )

}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}
