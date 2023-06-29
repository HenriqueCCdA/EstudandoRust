fn main() {
    conta_corrente();

    let notas: [f32; 4] = [6.5; 4];
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }

    matriz();
    print!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    cores();
    conteudo_opcional();
    vectors();

}

struct Conta {
    titular: Titular,
    saldo: f64,
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String,
}

fn conta_corrente(){

    let titular = Titular{
        nome: String::from("Vinicius"),
        sobrenome: String::from("Dias")
    };

    let mut conta: Conta = Conta{
        titular,
        saldo: 100.0
    };

    conta.sacar(50.0);

    println!("Dados da conta: Titula: {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}


fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };

    println!("{:?}", conteudo_arquivo);


    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza de que há um valor {}", valor)
    }
}


fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    // None
}


#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ =>  false
    }
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4],
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn cores() {

    let cor = Color::CymkColor{cyan: 100, magenta: 50, yellow: 70, black: 255};

    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0, 0, 0) | Color::CymkColor{cyan: _, magenta: _, yellow: _, black: 255} => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor{cyan: _, magenta: _, yellow: _, black: _} => "CYMK desconhecido"
    });
}

fn vectors() {

    let notas3:Vec<f32> = Vec::with_capacity(4);
    println!("Capacidade pre = {}", notas3.capacity());


    let mut notas: Vec<f32> = Vec::new();
    println!("Capacidade 1 = {}", notas.capacity());

    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);
    println!("Capacidade 2 = {}", notas.capacity());

    println!("{:?}", notas);

    notas.push(6.8);
    println!("Capacidade 3= {}", notas.capacity());

    println!("{:?}", notas);

    let notas2: Vec<f32> = vec![10.0, 8.0, 6.5];


    println!("{:?}", notas2);

    println!("Nota 1 = {}", notas[0]);

    println!("Nota 6 = {}", match notas.get(7) {
        Some(n) => *n,
        None => 0.0
    });

    if let Some(nota) = notas.pop() {
        println!("Ultimo valor = {}", nota);
    }
    println!("{:?}", notas);

    for nota in &notas {
        println!("Nota = {}", nota)
    }

    println!("{:?}", notas);


}
