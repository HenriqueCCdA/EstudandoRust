use std::error::Error;
use std::io;
use std::fs::File;
use std::io::Read;

#[allow(unused_variables)]
#[allow(unused_assignments)]

// fn read_username_from_file() -> Result<String, io::Error> {

//     let f = File::open("username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s){
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}



fn main() {

    let a = read_username_from_file();
    println!("{:?}", a);

}
