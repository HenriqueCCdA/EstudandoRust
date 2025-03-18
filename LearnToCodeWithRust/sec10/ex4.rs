#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(Credentials ),
}

fn main() {
    let paypal_credentials = Credentials {
        username: String::from("bob@gmail.com"),
        password: String::from("Password"),
    };

    let paypal = PaymentethodType::PayPal(paypal_credentials);

    println!("{:#?}", paypal)
}
