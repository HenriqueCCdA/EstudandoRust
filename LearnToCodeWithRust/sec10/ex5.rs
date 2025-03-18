#[derive(Debug)]
enum PaymentethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal { username: String, password: String},
}

fn main() {

    let paypal = PaymentethodType::PayPal{
        username: String::from("bob@gmail.com"),
        password: String::from("Password"),
    };

    println!("{:#?}", paypal)
}
