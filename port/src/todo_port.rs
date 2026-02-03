use domain::Todo;

#[async_trait::async_trait]
pub trait TodoPort: Send + Sync {
    async fn get_todos(&self) -> anyhow::Result<Vec<Todo>>;
}
