fn main() {
    let v = vec![100, 332, 57];

    for i in &v {
        println!("{i}");
    }
    println!("{:?}", v);
    
    let mut v1 = vec![100, 332, 57];

    for i in &mut v1 {
        *i += 50;
    }
    println!("{:?}", v1);

}
