use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // spawn the app
    spawn_app();

    // Bring the reqwest client
    let client = reqwest::Client::new();

    // send the get request
    let response = client
        .get("http://127.0.0.1:0/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // check
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Need to spawn the app
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port");
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener).expect("Failed to bind the address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
