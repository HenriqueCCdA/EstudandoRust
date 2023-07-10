static mut STATIC_VARIAVEL: i32 = 15;

fn main() {

    unsafe{
        println!("O valor da STATIC_VARIAVEL é {}", STATIC_VARIAVEL);
    }

}
