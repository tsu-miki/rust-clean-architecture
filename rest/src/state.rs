use std::sync::Arc;

use gateway::todo_gateway::TodoGateway;
use usecase::get_todos_usecase::GetTodosUsecase;

#[derive(Clone)]
pub struct AppState {
    pub get_todos_usecase: Arc<GetTodosUsecase>,
}

pub fn create_app_state() -> AppState {
    let todo_gateway = Arc::new(TodoGateway::new());
    let get_todos_usecase = Arc::new(GetTodosUsecase::new(todo_gateway));

    AppState { get_todos_usecase }
}
