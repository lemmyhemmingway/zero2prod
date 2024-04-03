use std::net::TcpListener;

use sqlx::PgPool;
use zerotoprod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Couldn't connect postgres");
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
