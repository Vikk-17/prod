use prod::startup;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_test() {
    // start the app
    let address = spawn_app();

    // create the client
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute the request");

    assert!(response.status().is_success()); // <- to check whether this is true
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // start the app
    let app_address = spawn_app();

    // create the client
    let client = reqwest::Client::new();
    let params = [("name", "Test Prod"), ("email", "test@gmail.com")];
    // let body = "name=Test%20Prod&email=test%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        // .body(body)
        .send() // <- Constructs the Request and sends it to the target URL, returning a future Response
        .await
        .expect("Failed to execute the request.");

    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();

    // create the client
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name%3DTest%20Prod", "missing email"),
        ("email%3Dtest%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute the request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload
was {}.",
            error_message
        )
    }
}

// run the program in the background
fn spawn_app() -> String {
    let listerner = TcpListener::bind("127.0.0.1:0").expect("Failed to random port");
    let port = listerner.local_addr().unwrap().port();
    let server = startup::run(listerner).expect("Failed to bind address");
    let server = prod::run(listerner).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
