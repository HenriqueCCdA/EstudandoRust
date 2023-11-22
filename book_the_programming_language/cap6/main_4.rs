fn main() {

    let config_max = Some(4.3f32);

    match config_max {
        Some(max) => println!("The maximun is configured to be {}", max),
        _ => (),
    }

}
