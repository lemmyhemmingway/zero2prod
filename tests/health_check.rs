use std::net::TcpListener;




#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
    .get(format!("http://{address}/healthcheck"))
        .send()
        .await
    .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    
    
}

fn spawn_app() -> String{ 
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    dbg!("Running on {port}");

    format!("127.0.0.1:{port}")
}
