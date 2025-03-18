#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let gold_chet = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chet.capital_captain());

    let mut silver_chest = TreasureChest {
        captain: String::from("Booldsail"),
        treasure: String::from("    Silver    "),
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootylunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest);

}
