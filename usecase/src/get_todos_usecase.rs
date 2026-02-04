use std::sync::Arc;

use domain::todo::Todo;
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

#[cfg(test)]
mod tests {
    use super::*;
    use port::todo_port::MockTodoPort;
    use rstest::fixture;

    struct Dependencies {
        todo_port: MockTodoPort,
    }

    #[fixture]
    fn dependencies() -> Dependencies {
        Dependencies {
            todo_port: MockTodoPort::new(),
        }
    }

    #[rstest::rstest]
    #[tokio::test]
    async fn test_execute(mut dependencies: Dependencies) {
        let todos = vec![Todo {
            id: "1".to_string(),
            name: "Todo 1".to_string(),
        }];

        dependencies
            .todo_port
            .expect_get_todos()
            .return_once(move || Ok(todos))
            .times(1);

        let usecase = GetTodosUsecase::new(Arc::new(dependencies.todo_port));

        let actual = usecase.execute().await.unwrap();
        let expected = vec![Todo {
            id: "1".to_string(),
            name: "Todo 1".to_string(),
        }];
        assert_eq!(actual, expected);
    }
}
