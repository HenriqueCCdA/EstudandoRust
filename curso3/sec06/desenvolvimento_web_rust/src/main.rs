#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "'Hello, Rocket!"
}

#[catch(404)]
fn not_found() -> &'static str {
    "Página não encontrada :("
}

fn main() {
    rocket::ignite()
        .mount("/",  routes![index])
        .register(catchers![not_found])
        .launch();
}

// #[get("/users/<id>")]
// fn get_user(id: u32) -> String {
//     format!("Retornado o usuário comm ID: {}", id)
// }

// #[post("/users/<name>")]
// fn create_user(name: String) -> String{
//     format!("Criando usuário com nome: {}", name)
// }


// #[delete("/users/<id>")]
// fn delete_user(id: u32) -> String {
//     format!("Deletando usuário de id: {}", id)
// }

// #[put("/users?<id>&<name>")]
// fn update_user(id: u32, name: String) -> String{
//     format!("Atualizando o usuário de id: {} para nome {}", id, name)
// }

// #[get("/users?<query>&<page>")]
// fn search_users(query: String, page: Option<u32>) -> String {
//     match page {
//         Some(p) => format!("Buscando usuários com a consulta '{}' na página {}", query, p),
//         None => format!("Buscando usuário com a consulta '{}' (sem especificar a página)", query)
//     }
// }


// fn main() {
//     rocket::ignite().mount("/", routes![index, get_user, create_user, delete_user, update_user, search_users]).launch();
// }

// ********************************************************************************************

// #[get("/hello/<name>")]
// fn hello(name: String) -> String {

//     format!("Hello, {}", name)
// }

// #[get("/number/<number>")]
// fn number(number: i32) -> String {
//     format!("O número é: {}", number)
// }

// #[get("/search?<query>&<max_results>&<page>")]
// fn search(query: String, max_results: i32, page: i32) -> String {
//     format!("Shearching for '{}' (max:{}, page:{})", query, max_results, page)
// }

// #[get("/search2?<query>&<typ>")]
// fn search2(query: String, typ: Option<String>) -> String {
//     match typ{
//         Some(t) => format!("Searching for '{}' (type:{})", query, t),
//         None => format!("Searching for '{}' (no type specified)", query)
//     }

// }
