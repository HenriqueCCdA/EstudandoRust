use std::io;

fn main(){

    let mut mensagem_usuario = String::new();

    println!("Ei usuario. Digite Algo:\n\n");

    match io::stdin().read_line(&mut mensagem_usuario) {
        Ok(_) => println!("Sucesso, Vc digitou {}", mensagem_usuario.to_uppercase()),

        Err(e) => println!("Deu erro. O Erro é {}", e)
    }

}
