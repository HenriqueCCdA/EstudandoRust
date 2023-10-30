#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let j = 5;
    let i = j;

    println!("{}", j);
    println!("{}", i);

    let v = vec![1, 2, 3, 4, 5];
    // let w = v;

    // println!("{:?}", w);

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };

    let v = foo(v);

    println!("{:?}", v);
 }
