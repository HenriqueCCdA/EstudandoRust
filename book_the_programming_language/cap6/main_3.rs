fn main() {

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {println!("1")}
fn remove_fancy_hat() {println!("2")}
fn move_player(num_spaces: u8) {println!("3")}
fn reroll() {println!("3")}
