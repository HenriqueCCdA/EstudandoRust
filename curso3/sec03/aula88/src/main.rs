extern crate rand;
use rand::Rng;

fn main() {

    let valores_radomicos = rand::thread_rng().gen_range(5., 11.);

    println!("{}", valores_radomicos);

}
