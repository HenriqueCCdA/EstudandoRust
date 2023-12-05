// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn server_order() {}
//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurante() {
//     crate::front_of_house::hosting::add_to_waitlist();

//     front_of_house::hosting::add_to_waitlist();
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurante() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// }

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}
