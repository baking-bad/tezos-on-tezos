use actix_web::{get, web, App, HttpServer, Responder};

#[get("/chains/main/blocks/head")]
async fn head() -> impl Responder {
    format!("todo")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new().service(head)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
