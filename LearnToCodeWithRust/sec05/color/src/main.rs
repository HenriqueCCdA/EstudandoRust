use core::num;

#[allow(dead_code)]

fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_2(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_iterative(number: i32) -> i32 {
    let mut product = 1;
    let mut count = number;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product

}

fn factorial_recursive(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }
    number * factorial_recursive(number - 1)
}

fn main() {
    println!("{}", color_to_number_2("red"));
    println!("{}", color_to_number_2("green"));
    println!("{}", color_to_number_2("blue"));
    println!("{}", color_to_number_2("purple"));

    println!("{}", factorial_iterative(5));
    println!("{}", factorial_recursive(5));

}
