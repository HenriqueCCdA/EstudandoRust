fn main() {
    condicionais();
    repeticoes();

    let  linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O proposito de {} eh {}", linguagem, proposito)
}

fn repeticoes() {
    let multiplicador:u8 =  5;
    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10 {
            break;
        }
    }

    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn condicionais() {
    let idade: u8 = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;


    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };

    println!("É {} de idade", condicao)
}
