use actix_web::{HttpResponse, Responder, get};

#[get("/healthcheck")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
