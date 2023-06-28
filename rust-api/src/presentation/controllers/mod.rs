use actix_web::{get, HttpResponse, Responder};

mod user;
mod todo;

pub use user::*;
pub use todo::*;

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
