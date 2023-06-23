fn main() {

    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let  booleana = false;
    println!("Tamanho booleana = {}", std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char= {}", std::mem::size_of_val(&letra));

}
