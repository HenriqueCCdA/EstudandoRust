fn calcula_media(notas: &[f32]) -> f32 {
    let tamanho = notas.len();
    let mut soma = 0.0;

    for nota in notas {
        soma += nota;
    }
    soma  / tamanho as f32
}


fn main() {
    let notas = [7.5, 8.0, 9.0, 6.5];
    let media = calcula_media(&notas);
    println!("A media das notas é: {}", media);
}
