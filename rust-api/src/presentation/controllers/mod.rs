use actix_web::{get, HttpResponse, Responder};

mod user;
pub use user::*;

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
