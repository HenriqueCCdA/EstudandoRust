#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::{State};
use std::collections::HashMap;

struct Local{
    temperatura: f64,
    cidade: String,
}

fn obter_temperatura(local: &Local) -> String {
    local.temperatura.to_string()
}

#[get("/")]
fn index() -> &'static str {
    "Bem-Vindo a nossa API!"
}

#[get("/temperatura/<cidade>")]
fn temperatura(local_state: State<Local>, cidade: String) -> content::Html<String>{
     let local = local_state.inner();

     if local.cidade == cidade{
        content::Html(format!("A temperatura da cidade: {} é de {}°C", cidade,  obter_temperatura(local)))
     } else {
        content::Html(format!("Não foi possivel obter a temperatura da cidade {}", cidade))
     }
}

fn main() {

    let mut local_state = HashMap::<String, Local>::new();
    local_state.insert("Sao Paulo".to_string(), Local{temperatura: 25.0, cidade: "Sao Paulo".to_string()});
    local_state.insert("Rio de Janeiro".to_string(), Local{temperatura: 40.0, cidade: "Rio de Janeiro".to_string()});
    local_state.insert("Belo Horizonte".to_string(), Local{temperatura: 30.0, cidade: "Belo Horizonte".to_string()});

    rocket::ignite()
        .mount("/", routes![index, temperatura])
        .manage(local_state)
        .launch();
}
