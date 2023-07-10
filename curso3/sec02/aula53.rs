struct User {
    username: String,
    email: String,
    ativo: bool,
    genero: String,
}


fn main() {

    let mut pessoa = User {
        username: String::from("JoaoPessoa"),
        email: String::from("joaoPessoa@gmail.com"),
        ativo: true,
        genero: String::from("Homem")
    };

    pessoa.ativo = false;

    println!("O nome do usuario é {}, seu email é {} e seu genero é {}", pessoa.username, pessoa.email, pessoa.genero);

}
