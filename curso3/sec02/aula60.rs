struct User (String, String, bool, String);

// fn user(usuario: &User) {
//     println!("O nome do usuario {}", usuario.username);
// }

fn main() {

    let mut pessoa = User(
        String::from("JoaoPessoa"),
        String::from("JoaoPessoa@gmail.com"),
        true,
        String::from("JoaoPessoa")
    );
    println!("O nome do usuário é {}, seu email é {}. A conta esta  ativa ? {}", pessoa.0, pessoa.1, pessoa.2);

    pessoa.1 = String::from("JoaoPessoa123");
}
