use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::{
    data::{
        repositories::UpdateTodoDto,
        services::{create_todo_service, list_todo_service, update_todo_service},
    },
    domain::entities::Todo,
    infra::repositories::TodoRepository,
};

#[post("/todos/{user_id}")]
pub async fn create_todo(
    repo: Data<TodoRepository>,
    todo: Json<Todo>,
    user_id: Path<i32>,
) -> impl Responder {
    if let Err(e) =
        create_todo_service(repo.into_inner(), todo.into_inner(), user_id.into_inner()).await
    {
        eprintln!("ERROR: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[get("/todos/{user_id}")]
pub async fn list_todo(repo: Data<TodoRepository>, user_id: Path<i32>) -> impl Responder {
    match list_todo_service(repo.into_inner(), user_id.into_inner()).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[put("/todos/{todo_id}")]
pub async fn update_todo(
    repo: Data<TodoRepository>,
    new_todo: Json<UpdateTodoDto>,
    user_id: Path<i32>,
) -> impl Responder {
    if let Err(e) = update_todo_service(
        repo.into_inner(),
        new_todo.into_inner(),
        user_id.into_inner(),
    )
    .await {
        eprintln!("ERROR: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}
