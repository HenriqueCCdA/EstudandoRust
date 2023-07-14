fn conta_maisculas(texto: &str) -> u32 {

    let mut contador = 0;

    for caracter in texto.chars() {
        if caracter.is_uppercase() {
            contador+=1;
        }
    }
    contador
}


fn main() {
    let texto = "Este é um teXto Exemplo";
    let contador = conta_maisculas(texto);

    println!("Número de letras maiusculas: {}", contador);


}
