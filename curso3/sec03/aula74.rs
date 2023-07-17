use std::fs::File;
use std::io::prelude::*;


fn main() {

    let mut arquivo = File::open("rust_wiki.txt").expect("nao consiguiu ler o arquivo");
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo).expect("Não conseguiu ler o arquivo e alocar na variavel conteudo.");

    println!("O conteudo em arquivo é :\n\n{}", conteudo);
}
