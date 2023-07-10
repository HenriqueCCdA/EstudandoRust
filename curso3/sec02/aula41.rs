enum Pagamento{
    Dinheiro(f32),
    CreditoCartao(bool, f32),
    DebitoCartao(bool, f32)
}


fn main() {

    let pessoa_pagamento = Pagamento::CreditoCartao(false, 100f32);

    match pessoa_pagamento {
        Pagamento::Dinheiro(f) => println!("A pessoa pagou em Dinheiro {} reais!", f),
        Pagamento::CreditoCartao(true, x) => println!("A pessoa pagou em Cartão de Credito {} reais!", x),
        Pagamento::CreditoCartao(false, x) => println!("O pagamento em Cartão de Credito não funcionou {} reais", x),
        _ => {}
    }


}
