fn main() {
    let month_days = 1..31;

    println!("{month_days:?}");

    let month_days = 1..=31;

    println!("{month_days:?}");

    for number in month_days {
        println!("{number}")
    }

    let letters = 'b'..'f';

    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Yellow"];

    for color in colors {
        println!("{color} is a great color!");
    }
}
