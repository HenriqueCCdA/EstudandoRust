extern crate rand;
use rand::Rng;
use std::io;

fn main() {

    println!("Bem-Vindo aao jogo de Adivinhaçãao de Palavras! Dica: Frutas!");

    let words = vec!["banana", "abacate", "uva", "laranja", "cacau", "caqui"];
    let secret_word = words[rand::thread_rng().gen_range(0, words.len())].to_string();
    let mut current_word = vec!['_'; secret_word.len()];

    loop {
        println!("Palavra corrente: {}", current_word.iter().collect::<String>());
        println!("Adivinha um letra:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next().unwrap();

        let mut found = false;

        for (i, c) in secret_word.chars().enumerate() {
            if c == guess {
                current_word[i] = c;
                found = true;
            }
        }

        if !found {
            println!("Letra não encotrada.");
        }
        if current_word.iter().collect::<String>() == secret_word {
            println!("Parabens, você adivinhou a palavrra {}", secret_word);
            break;
        }
    }
}
