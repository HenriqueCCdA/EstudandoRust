#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket::response::Content;
use rocket::http::ContentType;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct FormInput {
    username: String,
    password: String,
}


#[get("/form")]
fn form() -> Content<String> {

    let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title> Formulário </title>
            </head>
            <body>
                <h1> Exemplo de Formulário </h1>
                <form method="post" action="/form">
                    <div>
                        <label for="name">Nome</label>
                        <input type="text" name="name">
                    </div>
                    <div>
                        <label for="email">E-mail</label>
                        <input type="email" name="email">
                    </div>
                    <button type="submit">Enviar</button>
                </form>
            </body>
        </html>
    "#;

    Content(ContentType::HTML, html.to_string())
}


#[get("/")]
fn index() -> String {
    format!("Bem-Vindo a nossa API!")
}

#[post("/form", data="<form_input>")]
fn submit_form(form_input: Json<FormInput>) -> String {
    format!("Username: {}\n Password: {}", form_input.username, form_input.password)
}

fn main() {
    rocket::ignite().mount("/", routes![index, form, submit_form]).launch();
}
