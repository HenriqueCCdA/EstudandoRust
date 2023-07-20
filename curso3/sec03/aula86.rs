fn main() {

    {
        let minha_string = String::from("Oi meu nome é João");

        println!("{}", minha_string);
        println!("{}", minha_string.replace("João", "Henrique"));
    }

    {
        let minha_string = String::from("Fui hoje\nao mecado \ncomprar arroz");

        for i in  minha_string.lines() {
            println!("( {} )", i);
        }
    }

    {
        let minha_string = String::from("Minha+sogra+é+muito+feliz");
        let token: Vec<&str> = minha_string.split("+").collect();

        println!("{:?}", token);
        println!("{}", token[1]);

    }

    {
        let minha_string = String::from("    Meu nomme é João    ");
        println!("{}", minha_string);
        println!("{}", minha_string.trim());
    }

    {
        let minha_string =  String::from("Deixa um avaliacao de 5  estrelas");
        match minha_string.chars().nth(6) {
            Some(c) => println!("Sucesso ! o caracter  da 6 posicao é {}", c),
            None => println!("Erro. Não existe o caracter da 6 posicao"),
        }
    }


}
