fn main() {
    ownership();

    pattern_matching();

    erros();

}

fn ownership() {
    let mut uma_string = String::from("Vinicius");
    rouba(&mut uma_string);

    println!("{}", uma_string)
}

fn rouba(string: &mut String) {
    string.push_str(" Dias");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x{
            1 => "Pouco",
            2 | 3  => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}

fn erros() {
    match resultado() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero)
    };
}

fn resultado() -> Result<String, u8> {
    Ok(String::from("Tudo deu certo"))
}
