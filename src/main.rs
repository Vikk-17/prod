use prod::*;

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
