fn main() {
    let numero = 5;

    match numero {
        1 => println!("O número é 1"),
        2 | 3 => println!("O número é 2 ou 3"),
        4 ...10 => println!("O número está enter 4 e 10"),
        _ => println!("Eu não sei que numero é esse")
    }
}
