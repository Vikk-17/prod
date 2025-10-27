use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use std::net::TcpListener;


async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn post_handler(bytes: web::Bytes) -> impl Responder {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => Ok(format!("Testing your name: {}", text)),
        Err(_) => Err(HttpResponse::BadRequest().into()),
    } 
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/handler", web::post().to(post_handler))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
