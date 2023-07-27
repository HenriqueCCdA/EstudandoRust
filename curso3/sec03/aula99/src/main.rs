mod finance;
use finance::{Asset, Portfolio};

fn main() {
    let mut portfolio = finance::Portfolio::new();
    portfolio.add_asset(finance::Asset{
        name: "AAPL".to_string(),
        price: 135.0,
        quantity: 5,
    });
    portfolio.add_asset(finance::Asset{
        name: "GOOG".to_string(),
        price: 1235.0,
        quantity: 2,
    });

    println!("Valor total do portfólio: ${:.2}", portfolio.total_value())
}
