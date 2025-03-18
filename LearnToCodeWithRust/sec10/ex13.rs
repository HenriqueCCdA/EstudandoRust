enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
}

fn main() {
    let my_beverage = Milk::NonDiary {
        kind: String::from("Oat"),
    };

    if let Milk::NonDiary { kind } = my_beverage {
        println!("You beverage is {kind} milk");
    }

}
