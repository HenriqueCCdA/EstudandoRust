use std::io;

fn main() {
    loop {
        println!("Bem vindo ao jogo da adivinhação");
        println!("i - Iniciar o jogo");
        println!("q - Fechar o jogo");

        let mut escolha_str = String::new();
        io::stdin()
            .read_line(&mut escolha_str)
            .expect("Erro ao receber sua escolha");

        println!("A sua escolha foi {}", escolha_str);

        match escolha_str.trim().to_lowercase().as_str() {
            "i" => {
                game();
                continue;
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

fn game() {
    println!("Iniciando Jogo!");
    let mut pontuacao: u16 = 1000;
    let numero_alvo: u8 = 42;
    println!("Por favor digite o número que você acredita ser:");
    let mut chute = String::new();
    io::stdin()
        .read_line(&mut chute)
        .expect("Erro ao receber o número");

    let chute: u8 = match chute.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Valor não é válido ou não está entre 0 e 255");
            0
        }
    };
    check_win_condition(&mut pontuacao, &numero_alvo, &chute);
    println!(
        "A sua pontuação foi {}, e o número era {}",
        pontuacao, numero_alvo
    );
}

fn check_win_condition(pontuacao: &mut u16, numero: &u8, chute: &u8) {
    if chute < numero {
        *pontuacao -= 100;
    }
    println!("A pontuação foi {}, e p número era {}", pontuacao, numero);
}

#[test]
fn test_jogador_de_um_numero_errado_deve_dimunuir_pontuacao_geral() {
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 1;

    check_win_condition(&mut pontuacao, &numero, &chute);

    assert_eq!(pontuacao, 900);
}
