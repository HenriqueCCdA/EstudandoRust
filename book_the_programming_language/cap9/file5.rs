use std::fs::File;


fn main() {
    let greeting_file = File::open("hello.txt").expect("hello.txt should be include in this project");
}
