


#[actix_web::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
    .get("http://127.0.0.1:8000/healthcheck")
        .send()
        .await
    .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    
    
}

fn spawn_app() {
    let server = zerotoprod::run().expect("Failed to bind address");

    _ = tokio::spawn(server);
}
