use crate::Suit::{Heart, Spade, Club, Dianmond};

#[allow(unused_variables)]
#[allow(unused_assignents)]

fn main() {
    print_choice(Heart);
    print_choice(Club);
    print_choice(Dianmond);
    print_choice(Spade);

    country(44);
    country(34);
    country(125);
    country(-15);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1...999 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Dianmond
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => { println!("\u{2665}") }
        Spade => { println!("\u{2660}") }
        Club => { println!("\u{2663}") }
        Dianmond => { println!("\u{2666}") }
    }
}
