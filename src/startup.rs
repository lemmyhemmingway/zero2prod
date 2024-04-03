use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer, Result};
use sqlx::PgPool;

use crate::routes;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(routes::health_check)
            .service(routes::subscribe)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
