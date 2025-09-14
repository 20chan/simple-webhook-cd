use actix_web::{App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::health)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
