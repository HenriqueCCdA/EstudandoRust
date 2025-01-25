fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}

fn main() {
    let action_hero = String::from("Arnold Scharzenegger");
    do_hero_stuff(&action_hero);
    let another_action_hero = "Sykvester Stallone";
    do_hero_stuff(&another_action_hero);
}
