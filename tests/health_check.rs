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

// run the program in the background
fn spawn_app() -> String {
    let listerner = TcpListener::bind("127.0.0.1:0").expect("Failed to random port");
    let port = listerner.local_addr().unwrap().port();
    let server = prod::run(listerner).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
