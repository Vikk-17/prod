use prod::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await // <- awaiting on the future
}
