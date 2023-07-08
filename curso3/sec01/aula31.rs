fn main() {
    println!("Por favor, digite um sequenciaa de números reais?");

    let mut input = String::new();

    //Le a sequencia de números do usuario

    std::io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    let numbers: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map( |x| x.parse::<i64>().expect("Por favor, insira números reais"))
        .collect();

    let mut sum: i64 = 0;

    for num in &numbers {
        if num % 2 == 0 {
            sum += num;
        }
    }
    println!("A soma dos números pares eh: {}", sum);
}
