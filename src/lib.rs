
use std::net::TcpListener;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscriptions(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();
   //  .await
    Ok(server)
}
