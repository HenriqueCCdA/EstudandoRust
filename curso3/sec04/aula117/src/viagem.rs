pub struct DadosPassageiros {
    pub nome: String,
    pub numero_passaporte: String,
    pub idade: u8,
}

pub struct DadosVoos{
    pub codigo_voo: String,
    pub partida: String,
    pub destino: String,
    pub data_partida: String,
    pub hora_partida: String,
}

pub fn adicionar_passageiro(
    dados_passageiros: &mut Vec<DadosPassageiros>,
    nome: String,
    numero_passaporte: String,
    idade: u8,
) {
    let passageiro = DadosPassageiros{nome, numero_passaporte, idade};

    dados_passageiros.push(passageiro);

}

pub fn adicionar_voo(
    dados_voos: &mut Vec<DadosVoos>,
    codigo_voo: String,
    partida: String,
    destino: String,
    data_partida: String,
    hora_partida: String,
) {
    let voo = DadosVoos{codigo_voo, partida, destino, data_partida, hora_partida};
    dados_voos.push(voo);
}

pub fn exibir_voos(dados_voos: &Vec<DadosVoos>){
    for voo in dados_voos {
        println!("Código do Voo: {}", voo.codigo_voo);
        println!("Partida: {}", voo.partida);
        println!("Destino: {}", voo.destino);
        println!("Data de Partida: {}", voo.data_partida);
        println!("Hora de Partida: {}\n", voo.hora_partida);
    }
}

pub fn exibir_passageiros(dados_passageiros: &Vec<DadosPassageiros>) {
    for passageiro in dados_passageiros {
        println!("Nome: {}", passageiro.nome);
        println!("Número de Passaporte: {}", passageiro.numero_passaporte);
        println!("Idade: {}\n", passageiro.idade);
    }
}
