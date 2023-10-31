use std::fs::File;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // let f = File::open("main.jpg");

    // match f {
    //     Ok(f) => {
    //         println!("file found {:?}", f);
    //     },
    //     Err(e) => {
    //         panic!()
    //     }
    // }
    let f = File::open("main.jpg").unwrap();

}
