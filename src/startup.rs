use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer, Result};
use sqlx::PgConnection;

use crate::routes;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {

    let connection = web::Data::new(connection);
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::health_check)
            .service(routes::subscribe)

            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
