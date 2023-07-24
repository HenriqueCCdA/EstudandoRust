mod operations;
use operations::{Operation, calculate};

fn main() {
    let add = Operation::Addition(5, 3);
    let sub = Operation::Subtraction(5, 3);
    let mult = Operation::Multiplication(5, 3);
    let div = Operation::Division(5, 3);
    let div_by_zero = Operation::Division(5, 0);

    println!("A soma de 5 e 3 é: {:?}", calculate(add).unwrap());
    println!("A subracao de 5 e 3 é: {:?}", calculate(sub).unwrap());
    println!("A multiplicao de 5 e 3 é: {:?}", calculate(mult).unwrap());
    println!("A divisao de 5 e 3 é: {:?}", calculate(div).unwrap());

    match calculate(div_by_zero) {
        Ok(result) => println!("A divisão de 5 por 0 eh: {}", result),
        Err(e) => println!("Error: {}", e),
    };

}
