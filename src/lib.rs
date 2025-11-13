use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

// match_info() is an unsafe option
// use into_inner or use struct that implements serde deserialization
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}", name)
}

pub async fn run() -> Result<(), std::io::Error> {
    println!("Server is running in http://localhost:3030/");

    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
