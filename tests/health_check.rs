use std::net::TcpListener;
use zerotoprod::startup::run;


 #[actix_web::test]
 async fn health_check_works() {
    // arrange
    let address = spawn_app();
     let client = reqwest::Client::new();

     // Act
     let response = client.get(&format!("{}/healthcheck", &address))
                        .send()
                        .await
                        .expect("Failed to reach server");
     assert!(response.status().is_success());
     assert_eq!(Some(0), response.content_length());
 }

#[actix_web::test]
async fn subscribe_returns_200_after_valid_form_data() {
   // arrange
   let app_address = spawn_app();
   let client = reqwest::Client::new();

   // act
   let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
   let response = client
      .post(&format!("{}/subscriptions", &app_address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
      .expect("Failed to execute request");

   // Assert
   assert_eq!(200, response.status().as_u16());

}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
   // arrange
   let app_address = spawn_app();
   let client = reqwest::Client::new();
   let test_cases = vec![
      ("name=le%20guin", "missing email"),
      ("email=ursula_le_guin%40gmail.com", "missing name"),
      ("", "missing both email and name")
   ];

   for (invalid_body, error_message) in test_cases {
      let response = client
                  .post(&format!("{}/subscriptions", &app_address))
                  .header("Content-Type", "application/x-www-form-urlencoded")
                  .body(invalid_body)
                  .send()
                  .await
                  .expect("Failed to execute request");
                  
      assert_eq!(
         400,
         response.status().as_u16(),
         "The API did not fail with 400 Bad Request when the payload was {}.",
         error_message,
      )
   }
}

 fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to run the server");
    let port =  listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind the address");

    let _ = actix_web::rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
 }
