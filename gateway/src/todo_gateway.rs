use domain::todo::Todo;
use port::todo_port::TodoPort;

pub struct TodoGateway;

impl TodoGateway {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl TodoPort for TodoGateway {
    async fn get_todos(&self) -> anyhow::Result<Vec<Todo>> {
        Ok(vec![
            Todo {
                id: "1".to_string(),
                name: "Sample Todo 1".to_string(),
            },
            Todo {
                id: "2".to_string(),
                name: "Sample Todo 2".to_string(),
            },
        ])
    }
}
