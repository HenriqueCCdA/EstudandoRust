mod viagem;

use viagem::{DadosPassageiros, DadosVoos, adicionar_passageiro, adicionar_voo, exibir_passageiros, exibir_voos};

fn main() {
    let mut dados_passageiros: Vec<DadosPassageiros> = Vec::new();
    let mut dados_voos: Vec<DadosVoos> = Vec::new();

    adicionar_passageiro(&mut dados_passageiros, String::from("João"), String::from("123ABC"), 18);
    adicionar_passageiro(&mut dados_passageiros, String::from("Maria"), String::from("456DEF"), 18);
    adicionar_passageiro(&mut dados_passageiros, String::from("Pedro"), String::from("789GHI"), 18);

    adicionar_voo(
        &mut dados_voos,
        String::from("Voo101"),
        String::from("São Paulo"),
        String::from("Rio de Janeiro"),
        String::from("30/06/2020"),
        String::from("09:00"),
    );

    adicionar_voo(
        &mut dados_voos,
        String::from("Voo102"),
        String::from("Rio de Janeiro"),
        String::from("São Paulo"),
        String::from("01/07/2020"),
        String::from("09:00"),
    );

    exibir_passageiros(&dados_passageiros);

    exibir_voos(&dados_voos);
}
