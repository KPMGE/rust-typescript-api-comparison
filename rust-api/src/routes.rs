use actix_web::{get, HttpResponse, Responder};
use crate::domain::entities::User;

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/test")]
pub async fn create_user() -> impl Responder {
    let user = User{ 
        name: "kevin".to_string(),
        email: "test@email.com".to_string(),
        password: "testpass".to_string()
    };

    println!("user: {:?}", user);

    HttpResponse::Ok().finish()
}
