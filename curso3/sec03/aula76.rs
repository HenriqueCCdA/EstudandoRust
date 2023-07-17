use std::fs::File;
use std::io::prelude::*;


fn main() {

    let mut arquivo = File::create("teste.txt").expect("Não conseguiu criar o arquivo");

    arquivo.write_all(b"Arquivo de teste sendo criado").expect("Não conseguiu escrever no arquivo");
}
