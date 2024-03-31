use actix_web::{post, web, Result};
use sqlx::PgConnection;



#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}


#[post("/subscriptions")]
pub async fn subscribe(_form: web::Form<FormData>, _connection: web::Data<PgConnection>) -> Result<String> {
    Ok(format!("{}", _form.name))
}
