use actix_web::{get, HttpResponse, Responder};

#[get("/healthcheck")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
