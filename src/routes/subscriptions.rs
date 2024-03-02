use actix_web::{post, web, Result};



#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}


#[post("/subscriptions")]
pub async fn subscribe(_form: web::Form<FormData>) -> Result<String> {
    Ok(format!("{}", _form.name))
}
