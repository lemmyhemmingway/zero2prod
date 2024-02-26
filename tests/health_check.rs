use std::net::TcpListener;


fn spawn_app() -> String{ 
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    dbg!("Running on {port}");

    format!("127.0.0.1:{port}")
}


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

#[actix_web::test]
async fn subscribe_returns_a_200_for_a_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
                    .post(&format!("{}/subscriptions", &address))
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(body)
                    .send()
                    .await
                    .expect("Failed to execute request");


    assert_eq!(200, response.status().as_u16())

}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
                    .post(&format!("{}/subscriptions", &address))
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(body)
                    .send()
                    .await
                    .expect("Failed to execute request");


    assert_eq!(200, response.status().as_u16())

}

