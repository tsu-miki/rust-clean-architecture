use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TodoJson {
    pub id: String,
    pub name: String,
}

pub struct TodoDriver {
    csv_path: PathBuf,
}

impl TodoDriver {
    pub fn new(csv_path: PathBuf) -> Self {
        Self { csv_path }
    }

    pub fn default_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("data")
            .join("todos.csv")
    }

    pub fn get_todos(&self) -> anyhow::Result<Vec<TodoJson>> {
        let mut reader = csv::Reader::from_path(&self.csv_path)?;
        let todos = reader.deserialize().collect::<Result<Vec<TodoJson>, _>>()?;
        Ok(todos)
    }
}

impl Default for TodoDriver {
    fn default() -> Self {
        Self::new(Self::default_path())
    }
}
