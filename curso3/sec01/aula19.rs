fn dobro(num: i32) -> i32 {
    return 2 * num;
}

fn maior(a: i32, b:i32) -> i32 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

fn main() {

    println!("O dobro do número 5 é {}", dobro(5));
    println!("O maior número entre 5 e 4 é {}", maior(5, 4));

}
