use rand::{thread_rng, Rng};
use std::io;

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Gaming,
    Lose,
}

fn main() {
    loop {
        println!("Bem vindo ao jogo da adivinhação");
        println!("i - Iniciar o jogo");
        println!("q - Fechar o jogo");

        let mut escolha_str = String::new();
        io::stdin()
            .read_line(&mut escolha_str)
            .expect("Erro ao receber sua escolha");

        match escolha_str.trim().to_lowercase().as_str() {
            "i" => {
                println!("Iniciando Jogo!");
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
    let mut pontuacao: u16 = 1000;
    let numero_alvo: u8 = thread_rng().gen_range(1..100);
    loop {
        println!("Por favor digite número que você acredita ser:");
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
        match check_win_condition(&mut pontuacao, &numero_alvo, &chute) {
            Ok(result) => {
                if result == GameResult::Win {
                    println!("Parabens você venceu! Sua pontuação foi {}", pontuacao);
                } else if result == GameResult::Lose {
                    println!("Que pena você perdeu!");
                    break;
                }
            }
            Err(_) => {
                println!("Ocorreu um erro e o jogo será reiniciado");
                break;
            }
        };
    }
}

fn check_win_condition(
    pontuacao: &mut u16,
    numero: &u8,
    chute: &u8,
) -> Result<GameResult, GameResult> {
    if chute == numero {
        return Ok(GameResult::Win);
    }
    if chute < numero {
        println!("O número secreto é maior");
        *pontuacao -= 100;
    }

    if chute > numero {
        println!("O número secreto é menor");
        *pontuacao -= 100;
    }
    if *pontuacao <= 0 {
        return Ok(GameResult::Lose);
    }
    println!("A pontuação foi {}, e p número era {}", pontuacao, numero);
    Ok(GameResult::Gaming)
}

#[test]
fn test_jogador_de_um_numero_errado_deve_dimunuir_pontuacao_geral() {
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 1;

    let result = check_win_condition(&mut pontuacao, &numero, &chute);

    assert_eq!(result, Ok(GameResult::Gaming));
    assert_eq!(pontuacao, 900);
}

#[test]
fn test_jogador_deu_numero_exato_deve_finaliza_jogo_sem_mudar_pontuacao() {
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 42;

    let result = check_win_condition(&mut pontuacao, &numero, &chute);

    assert_eq!(result, Ok(GameResult::Win));
    assert_eq!(pontuacao, 1000);
}

#[test]
fn test_jogador_deu_numero_errado_deve_finalizar_jogo_perdendo() {
    let mut pontuacao: u16 = 100;
    let numero: u8 = 42;
    let chute: u8 = 1;

    let result = check_win_condition(&mut pontuacao, &numero, &chute);

    assert_eq!(result, Ok(GameResult::Lose));

    assert_eq!(pontuacao, 0)
}

#[test]
fn test_jogador_deu_numero_errado_algo_deve_finalizar_jogo_perdendo() {
    let mut pontuacao: u16 = 100;
    let numero: u8 = 42;
    let chute: u8 = 100;

    let result = check_win_condition(&mut pontuacao, &numero, &chute);

    assert_eq!(result, Ok(GameResult::Lose));

    assert_eq!(pontuacao, 0)
}

#[test]
fn test_jogador_deu_numero_errado_pra_baixo_deve_diminuir_pontuacao_geral() {
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 1;

    let result = check_win_condition(&mut pontuacao, &numero, &chute);

    assert_eq!(result, Ok(GameResult::Gaming));

    assert_eq!(pontuacao, 900)
}

#[test]
fn test_jogador_deu_numero_errado_pra_cima_deve_diminuir_pontuacao_geral() {
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 100;

    let result = check_win_condition(&mut pontuacao, &numero, &chute);

    assert_eq!(result, Ok(GameResult::Gaming));

    assert_eq!(pontuacao, 900)
}
