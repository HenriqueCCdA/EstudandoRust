#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket::response::Content;
use rocket::http::{ContentType, Status};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct FormInput {
    name: String,
    email: String,
}

fn validade_input(form_input: &FormInput) -> Result<(), String> {
    if form_input.name.is_empty() {
        return Err("Name is  required".to_string());
    }

    if form_input.email.is_empty() {
        return Err("Email is required".to_string());
    }
    Ok(())
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
fn submit_form(form_input: Json<FormInput>) -> Result<Content<String>, Status>{
    match validade_input(&form_input) {
        Ok(_) => {
            let message = format!("Name: {}\nEmail: {}", form_input.name, form_input.email);
            let html = format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title> Formulário </title>
                </head>
                <body>
                    <h1> Successo! </h1>
                    <p>{}<\p>
                </body>
            </html>"#, message);
            Ok(Content(ContentType::HTML, html.to_string()))
        },

        Err(error) => {
            let html = format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title> Formulário </title>
                </head>
                <body>
                    <h1> Erro! </h1>
                    <p>{}<\p>
                    <p><a href="/form">Voltar<\a><\p>
                </body>
            </html>"#, error);
            Err(Status::UnprocessableEntity)
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, form, submit_form]).launch();
}
