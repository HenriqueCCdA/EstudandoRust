fn main() {
    let numeros_inteiros: [i32;5] = [1, 2, 3, 4, 5];

    println!("{}", numeros_inteiros[4]);

    for i in 0..numeros_inteiros.len() {
        println!("{}", numeros_inteiros[i]);
    }

    for n in numeros_inteiros.iter() {
        println!("{}", n);
    }
}
