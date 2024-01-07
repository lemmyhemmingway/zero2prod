use std::net::TcpListener;

use zerotoprod::run;

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind the address");

    run(listener)?.await
}

