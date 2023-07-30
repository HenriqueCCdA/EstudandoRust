use std::collections::HashMap;
use std::io;

struct User {
    name: String,
    age: u32,
    friends: Vec<String>,
}


fn main () {
    let mut users = HashMap::new();

    loop {
        println!("Escolha uma opção:");
        println!("1. Adicionar usuario");
        println!("2. Adicionar amigo");
        println!("3. Ver lista de amigos");
        println!("4. Sair");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                println!("Digite o nome:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                println!("Digite a idade");
                let mut age = String::new();
                io::stdin().read_line(&mut age).unwrap();
                let age: u32 = age.trim().parse().unwrap();

                let user = User{
                    name: name.clone(),
                    age,
                    friends: Vec::new(),
                };

                users.insert(name, user);
            },

            2 => {
                println!("Digite o nome do usuário:");
                let mut user_name = String::new();
                io::stdin().read_line(&mut user_name).unwrap();
                let user_name = user_name.trim().to_string();

                println!("Digite o nome do amigo:");
                let mut friend_name = String::new();
                io::stdin().read_line(&mut friend_name).unwrap();
                let friend_name = friend_name.trim().to_string();

                let user = users.get_mut(&user_name).unwrap();
                user.friends.push(friend_name);
            },

            3 => {
                println!("Digite o nome do usuário:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                let user = users.get(&name).unwrap();
                println!("Amigos de {}", name);
                for friend in &user.friends {
                    println!("- {}", friend);
                }
            },

            4 => {
                break;
            }

            _ => {
                println!("Opção inválida");
            }

        }

    }
}
