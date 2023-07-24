mod imobiliaria;
use imobiliaria::Imobiliaria;


fn main() {

    let mut imobiliaria = Imobiliaria{
        nome: String::from("Imobiliria ABC"),
        endereco: String::from("Rua dos bobos, 123"),
        imoveis: Vec::new(),
    };

    imobiliaria.novo_imovel(
        String::from("Rua dos bobos, 124"),
        200_000.00,
        3,
        2,
        150.0,
    );

    imobiliaria.listar_imoveis();
}
