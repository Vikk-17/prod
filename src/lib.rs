use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// match_info() is an unsafe option
// use into_inner or use struct that implements serde deserialization
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("world");
//     format!("Hello {}", name)
// }

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
