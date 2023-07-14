fn main() {
    let mut minhaString: String = String::from("Olá meu nome é Henrique");

    println!("O tamanho  dessa string é {}", minhaString.len());
    println!("A minha string esta vazia ? {}", minhaString.is_empty());

    for token in minhaString.split_whitespace() {
        println!("{}", token);
    }

    println!("O nome Henrqiue esta contrido na String? {} ", minhaString.contains("Henrique"));
    minhaString.push_str(" Bem-vindo a aula.");

    println!("{}", minhaString);
}
