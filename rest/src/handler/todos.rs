use axum::{Json, extract::State};
use serde::Serialize;

use crate::{error::HandlerResult, state::AppState};

#[derive(Serialize)]
pub struct Todo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct ToDoListResponse {
    pub todos: Vec<Todo>,
}

pub async fn list(State(app_state): State<AppState>) -> HandlerResult<Json<ToDoListResponse>> {
    let todos = app_state.get_todos_usecase.execute().await?;
    let response = ToDoListResponse {
        todos: todos
            .into_iter()
            .map(|t| Todo {
                id: t.id,
                name: t.name,
            })
            .collect(),
    };
    Ok(Json(response))
}
