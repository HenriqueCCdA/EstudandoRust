use std::io;

/// Create comment.
/// What is this module trying to achive.

fn main() {

    //! # Main function
    //!
    //! ```
    //! fn main()
    //! ```
    //!
    //! Reads user input and prints it to console.
    //!
    let mut input = String::new();

    // Print a message to the user
    println!("Say someting");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went worng {}", e);
        }
    }
}
