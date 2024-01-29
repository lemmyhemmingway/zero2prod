use std::net::TcpListener;

use actix_web::{HttpServer, App, web, dev::Server};

use crate::routes::{health_check, subscriptions};



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
