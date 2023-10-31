use std::io::{Read, Write};
use std::fs::{File, OpenOptions, remove_file};

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // let mut file = File::create("example.txt").expect("create failed");
    // file.write_all("Hello World!\n".as_bytes()).expect("write failed");

    // let mut file = OpenOptions::new().append(true)
    //     .open("example.txt")
    //     .expect("canno open file");

    // file.write_all("Adding content to te file.\n".as_bytes()).expect("write failed");

    // let mut file = File::open("example.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    remove_file("example.txt").expect("delete failed");
}
