#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
}

fn main() {
    let my_beverage = Milk::Lowfat(2);

    let Milk::Lowfat(percent) = my_beverage else {
        println!("You do not have the lowfat milk");
        return;
    };

    println!("{percent}% milk is availabel here");
}
