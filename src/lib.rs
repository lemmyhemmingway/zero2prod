use std::{hash::Hasher, net::TcpListener};

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder, Result};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> Result<String> {
    Ok(format!("{}", _form.name))
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
