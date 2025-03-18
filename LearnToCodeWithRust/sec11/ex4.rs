#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}


fn main() {
    let gold_chet = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chet);

    let silver_chest = TreasureChest {
        captain: String::from("Booldsail"),
        treasure: String::from("Silver"),
    };
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootylunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest);

}
