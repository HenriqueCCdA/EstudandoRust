#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input a miner address: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut miner_addr);
    println!("Difficulty:");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu:");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("0. Exit");
        let _ = io::stdout().flush();
        choice.clear();
        let _ = io::stdin().read_line(&mut choice);

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut sender);

                println!("Enter receiver address:");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut receiver);

                println!("Enter amount:");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed")
                }

            },
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed")
                }
            },
            3 => {
                let mut new_diff = String::new();
                println!("Please enter new difficulty");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match  res {
                    true => println!("Updated difficulty"),
                    false => println!("Failed update"),

                }
            },
            4 => {
                let mut new_reward = String::new();
                println!("Enter new reward:");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());

                match res {
                    true => println!("Updated rewward"),
                    false => println!("Failed update"),
                }
            },
            _ => println!("Invalid choise please try again")
        };
   }

}
