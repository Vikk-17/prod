use prod::*;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server is running on http://127.0.0.1:{}", port);
    run(listener)?.await // <- awaiting on the future
}
