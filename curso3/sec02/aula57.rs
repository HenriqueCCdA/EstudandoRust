fn eh_palindromo(frase: &str) -> bool {
    let frase_limpa = frase.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
    let tamanho = frase_limpa.len();

    for i in 0..tamanho/2{
        if frase_limpa.chars().nth(i) != frase_limpa.chars().nth(tamanho-i-1) {
            return false;
        }
    }

    true
}


fn main() {
    let frase = "A man, a plan, a canal: Panama";
    let resultado = eh_palindromo(frase);

    println!("A frase '{}' é palindromo? {}", frase, resultado);
}
