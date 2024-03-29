use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zerotoprod::configuration::{self, get_configuration};


fn spawn_app() -> String{ 
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}


#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
    .get(format!("{}/healthcheck", &address))
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
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
                    .await
                    .expect("Couldn't connect to database");
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
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(400, response.status().as_u16(), "400 request when payload {}", error_message)
    }

}

