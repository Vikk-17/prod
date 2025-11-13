#[tokio::test]
async fn health_check_test() {
    // start the app
    spawn_app();

    // create the client
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/health_check")
        .send()
        .await
        .expect("Failed to execute the request");

    assert!(response.status().is_success()); // <- to check whether this is true
    assert_eq!(Some(0), response.content_length());
}

// run the program in th background
fn spawn_app() {
    let server = prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
