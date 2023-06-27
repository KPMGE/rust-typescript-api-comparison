mod routes;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::health_check))
        .bind(("127.0.01", 3333))?
        .run()
        .await
}
