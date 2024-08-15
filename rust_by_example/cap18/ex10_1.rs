use std::num::ParseIntError;

fn mutiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>)  {
    match result {
        Ok(n) => println!("n in {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = mutiply("10", "2");
    print(twenty);

    let tt = mutiply("t", "2");
    print(tt);
}
