extern crate rand;
use rand::Rng;


fn gera_aleatorios() {
    for _  in 0..10{
        println!("{}", rand::thread_rng().gen_range(0, 101));
    }
}

fn main() {
    gera_aleatorios()
}
