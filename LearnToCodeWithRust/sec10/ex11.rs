#[derive(Debug)]
enum OnLineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnLineOrderStatus {
    fn check(&self) {
        match self {
            OnLineOrderStatus::Delivered => {
                println!("Your item has been delivered");
            }
            other_status => {
                println!("Your item is {other_status:?}");
            }
        }
    }
}

fn main() {
    OnLineOrderStatus::Ordered.check();
    OnLineOrderStatus::Delivered.check();
    OnLineOrderStatus::Shipped.check();
}
