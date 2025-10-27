use std::net::TcpListener;

use email_newsletter::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port");
    run(listener)?.await
}
