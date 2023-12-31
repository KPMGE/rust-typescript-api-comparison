use std::time::Duration;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use presentation::controllers::{
    create_todo, create_user, delete_todo, delete_user, health_check, list_todo, list_user,
    update_todo, update_user,
};

mod data;
mod domain;
mod infra;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(20))
        .connect_lazy(db_url.as_str())
        .expect("could not connect to the database!");

    let user_repo = infra::repositories::UserRepository::new(pool.clone());
    let todo_repo = infra::repositories::TodoRepository::new(pool);
    let repo_data = Data::new(user_repo);
    let todo_data = Data::new(todo_repo);

    HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(create_user)
            .service(list_user)
            .service(update_user)
            .service(delete_user)
            .service(create_todo)
            .service(list_todo)
            .service(update_todo)
            .service(delete_todo)
            .app_data(repo_data.clone())
            .app_data(todo_data.clone())
    })
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}
