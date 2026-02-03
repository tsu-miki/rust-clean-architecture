use axum::{Router, routing::get};

use crate::{
    handler::{systems, todos},
    state::AppState,
};

pub fn systems_router() -> Router<AppState> {
    Router::new().route("/v1/systems/ping", get(systems::ping))
}

pub fn todos_router() -> Router<AppState> {
    Router::new().route("/v1/todos", get(todos::list))
}
