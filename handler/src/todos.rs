use axum::Json;
use serde::Serialize;

use crate::error::HandlerResult;

#[derive(Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct ToDoListResponse {
    pub todos: Vec<Todo>,
}

pub async fn list() -> HandlerResult<Json<ToDoListResponse>> {
    Ok(Json(ToDoListResponse { todos: vec![] }))
}
