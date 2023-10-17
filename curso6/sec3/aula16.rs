#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    // for i in 1..6 {
    //     say_hi()
    // }

    // let mut name= "Jhon";
    // say_hello(&mut name);
    // println!("{}", name);

    let mut name= "Jhon";
    let  greeting = say_hello(&mut name);
    println!("{}", greeting);
}

// fn say_hi() {
//     println!("Hello there");
// }

// fn say_hello(name: &mut &str) {
//     *name = "Alex";
//     println!("Hello {}", name);
// }


fn say_hello(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    greeting
}
