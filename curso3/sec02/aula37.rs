#[derive(Debug)]
enum CarType{
    Fiat, Ford, Renault
}

fn nacionalidade_carro(car:CarType) {
    match car {
        CarType::Fiat => println!("O carro é italiano!"),
        CarType::Ford => println!("O carro é americano!"),
        CarType::Renault => println!("O carro é frances!"),
    }
}


fn main() {

    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Renault);

}
