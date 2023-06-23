const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 1;

fn sombra(){
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);

    }

    println!("fora, a = {}", a);
}


fn escopo() {
    println!("PI = {}", PI);
    println!("variavel_global = {}", VARIAVEL_GLOBAL);

    let variavel:i32 = 300;
    let variavel:i32 = 301;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let  booleana = false;
    println!("Tamanho booleana = {}", std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char= {}", std::mem::size_of_val(&letra));
}



fn main() {
    escopo();
    sombra();
}
