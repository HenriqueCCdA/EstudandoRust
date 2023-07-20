extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {

    println!("Adivinhe o número!");
    let  secret_number = rand::thread_rng().gen_range(0, 101);
    println!("secret_number => {}", secret_number);


    loop  {
        println!("Digite seu plapite:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você palpitou: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("baxio!"),
            Ordering::Greater => println!("alto!"),
            Ordering::Equal => {
                println!("Voce venceu!");
                break
            }
        }
    }

}
