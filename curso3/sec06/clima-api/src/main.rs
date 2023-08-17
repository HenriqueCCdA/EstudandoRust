#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::State;

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

    rocket::ignite().mount("/", routes![index, temperatura])
        .manage(Local{
            temperatura: 20.0,
            cidade: "Sao Paulo".to_string()
        })
        .launch();
}
