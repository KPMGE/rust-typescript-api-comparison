use std::time::Duration;

use actix_web::{web::Data, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod data;
mod domain;
mod infra;
mod presentation;

use presentation::controllers::{create_user, health_check, list_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "postgresql://postgres:root@localhost:5432/users?schema=public";
    let pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(20))
        .connect_lazy(db_url)
        .expect("could not connect to the database!");

    let user_repo = infra::repositories::UserRepository::new(pool);
    let repo_data = Data::new(user_repo);

    HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(create_user)
            .service(list_user)
            .app_data(repo_data.clone())
    })
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}
