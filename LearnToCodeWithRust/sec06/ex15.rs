fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    println!{"{current_meal}"}
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

fn add_sugar() {}
