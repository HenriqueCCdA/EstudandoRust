enum Pagamento{
    Dinheiro,
    CreditoCartao,
    DebitoCartao
}


fn main() {

    let pessoa_pagamento = Pagamento::CreditoCartao;

    match pessoa_pagamento {
        Pagamento::Dinheiro => println!("A pessoa pagou em Dinheiro!"),
        Pagamento::CreditoCartao => println!("A pessoa pagou em Cartão de Credito"),
        Pagamento::DebitoCartao => println!("A pessoa pagou no Cartão de Debito"),
    }


}
