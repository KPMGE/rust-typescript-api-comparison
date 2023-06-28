use crate::data::services::{create_user_service, CreateUserError::*};
use crate::domain::entities::User;
use crate::infra::repositories::UserRepository;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/users")]
pub async fn create_user(user: Json<User>, repo: Data<UserRepository>) -> impl Responder {
    if let Err(e) = create_user_service(&user, repo.into_inner()).await {
        eprintln!("ERROR: {:?}", e);
        return match e {
            InvalidUserError(e) => HttpResponse::BadRequest().json(e),
            InvalidEmailError(e) => HttpResponse::BadRequest().json(e),
            RepoError(e) => HttpResponse::InternalServerError().json(e),
        };
    }

    HttpResponse::Ok().finish()
}
