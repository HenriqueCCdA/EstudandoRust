fn main() {
    let coffee_price: f64 = 5.99;

    {
        let cookie_price = 1.99;
        println!("The price is {coffee_price}");
        println!("The price is {cookie_price}");
    }
    // println!("The price is {cookie_price}");

}
