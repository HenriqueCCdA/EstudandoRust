static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 0;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold os {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else{ "small" });


}
