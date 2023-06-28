use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::{
    data::services::create_todo_service, domain::entities::Todo,
    infra::repositories::TodoRepository,
};

#[post("/todos/{user_id}")]
pub async fn create_todo(repo: Data<TodoRepository>, todo: Json<Todo>, user_id: Path<i32>) -> impl Responder {
    if let Err(e) = create_todo_service(
        repo.into_inner(), 
        todo.into_inner(),
        user_id.into_inner()
    ).await {
        eprintln!("ERROR: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}
