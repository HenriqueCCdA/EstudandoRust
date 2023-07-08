fn main() {
    let tupla = (12, "valores", 3.14, (1, 2, 3));
    let (a, b, c, d) = tupla;
    println!("{}", (tupla.3).2);

    println!("valor de a é {}", a);
    println!("valor de b é {}", b);
    println!("valor de c é {}", c);
    println!("valor de d é {:?}", d);
}
