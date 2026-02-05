use domain::todo::Todo;
use driver::todo_driver::{TodoDriver, TodoJson};
use port::todo_port::TodoPort;

pub struct TodoGateway {
    driver: TodoDriver,
}

impl TodoGateway {
    pub fn new(driver: TodoDriver) -> Self {
        Self { driver }
    }
}

impl Default for TodoGateway {
    fn default() -> Self {
        Self::new(TodoDriver::default())
    }
}

fn to_todo(json: TodoJson) -> Todo {
    Todo {
        id: json.id,
        name: json.name,
    }
}

#[async_trait::async_trait]
impl TodoPort for TodoGateway {
    async fn get_todos(&self) -> anyhow::Result<Vec<Todo>> {
        let todos = self
            .driver
            .get_todos()?
            .into_iter()
            .map(to_todo)
            .collect::<Vec<Todo>>();
        Ok(todos)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_csv(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[tokio::test]
    async fn test_get_todos_returns_todos_from_csv() {
        let csv_content = "id,name\n1,Test Todo 1\n2,Test Todo 2\n";
        let temp_file = create_test_csv(csv_content);

        let driver = TodoDriver::new(temp_file.path().to_path_buf());
        let gateway = TodoGateway::new(driver);

        let todos = gateway.get_todos().await.unwrap();

        assert_eq!(
            todos,
            vec![
                Todo {
                    id: "1".to_string(),
                    name: "Test Todo 1".to_string(),
                },
                Todo {
                    id: "2".to_string(),
                    name: "Test Todo 2".to_string(),
                }
            ]
        );
    }
}
