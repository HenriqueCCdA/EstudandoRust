#[allow(unused_variables)]
#[allow(unused_assignments)]

macro_rules! my_macro {
    () => (println!("First macro"))
}

// macro_rules! name {
//     ($name: expr) => { println!("Hey {}", $name)}
// }

macro_rules! name {
    ($($name: expr),*) => ( $(println!("Hey {}", $name);)* )
}

macro_rules! xy {
    (x => $e: expr) => (println!("X is {}", $e));
    (y => $e: expr) => (println!("Y is {}", $e));
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    }
}

fn main() {
    my_macro!();

    name!("Jhon");
    name!("Alex", "Mary", "carol");
    xy!(x => 5);
    xy!(y => 3 * 9);
    build_fn!(hey);
    hey();
}
