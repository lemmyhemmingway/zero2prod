use actix_web::{HttpResponse, web, Responder};


#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}


pub async fn subscriptions(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
