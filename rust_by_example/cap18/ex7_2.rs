#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_availabel_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit os: {:?}", first_availabel_fruit);
    println!("my_fruit is: {:?}", my_fruit);
}
