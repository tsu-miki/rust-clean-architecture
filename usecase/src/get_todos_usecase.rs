use std::sync::Arc;

use domain::Todo;
use port::todo_port::TodoPort;

pub struct GetTodosUsecase {
    todo_port: Arc<dyn TodoPort>,
}

impl GetTodosUsecase {
    pub fn new(todo_port: Arc<dyn TodoPort>) -> Self {
        Self { todo_port }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<Todo>> {
        self.todo_port.get_todos().await
    }
}
