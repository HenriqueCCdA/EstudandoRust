use std::io;

fn main() {
    let mut pontuacao: i32 = 0;

    println!("Bem-vindo à floresta misteriosa!");

    println!("Escolha sua próxima ação");

    println!("1 - Entrar na caverna escura");
    println!("2 - Seguir pelo caminho iluminado");
    println!("3 - Cruzar a ponte frágil");
    println!("4 - Descansar na beira do riacho");

    let mut escolha_str = String::new();
    let _ = io::stdin().read_line(&mut escolha_str);

    let escolha: u32 = match escolha_str.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if escolha == 1 {
        pontuacao += 50;
    } else if escolha == 2 || escolha == 3 {
        pontuacao -= 20;
    } else if escolha == 4 {
        pontuacao += 10;
    }

    println!("Sua escolha foi {}", escolha);
    println!("Sua pontuacao foi {}", pontuacao);
}
