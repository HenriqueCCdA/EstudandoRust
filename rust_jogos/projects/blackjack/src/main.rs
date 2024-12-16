mod game;
mod card;

fn main() {
    loop {
        println!("Start game? (y)es (n)o");
        let mut option = String::new();
        std::io::stdin().read_line(&mut option).unwrap();
        match option.trim().to_ascii_lowercase().as_str() {
            "y" => {
                game::game_start();
                continue;
            }
            "n" => {
                println!("Thank you for play");
                break;
            }
            _ => {
                println!("Wrong option use 'y' or 'n'");
                continue;
            }
        }
        // game::game_start()
    }
}
