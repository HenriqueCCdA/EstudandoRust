fn main() {
    let nome = "hello";

    let nome2 = String::from("hello world");


    let hello =  &nome2[0..5];

    println!("{}", hello);
    println!("{}", nome);

}
