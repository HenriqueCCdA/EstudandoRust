use std::io;

fn main() {
    loop {
        println!("Bem vindo ao jogo da adivinhação");
        println!("i - Iniciar o jogo");
        println!("q - Fechar o jogo");

        let mut escolha_str = String::new();
        let _ = io::stdin().read_line(&mut escolha_str);

        println!("A sua escolha foi {}", escolha_str);

        match escolha_str.trim().to_lowercase().as_str() {
            "i" => {
                println!("iniciando Jogo!");
            }
            "q" => {
                println!("Obrigado por jogar!");
                break;
            }
            _ => {
                println!("Escolha inválida. Tente novamente.");
                continue;
            }
        }
    }
}
