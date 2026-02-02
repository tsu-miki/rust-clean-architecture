use axum::{routing::get, Router};
use handler::{systems, todos};

pub fn systems_router() -> Router {
    Router::new().route("/v1/systems/ping", get(systems::ping))
}

pub fn todos_router() -> Router {
    Router::new().route("/v1/todos", get(todos::list))
}

