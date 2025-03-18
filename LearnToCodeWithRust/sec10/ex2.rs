#[derive(Debug)]
enum PaymmentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
}

fn main() {
    let visa = PaymmentMethodType::CreditCard(String::from("0034-5678"));
    let matercard = PaymmentMethodType::CreditCard(String::from("2553-5678"));
    println!("{:?}", visa);
    println!("{:?}", matercard);
}
