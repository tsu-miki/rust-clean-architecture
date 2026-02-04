use axum::{Router, routing::get};

use crate::{
    handler::{systems::get_ping, todos::get_todos},
    state::AppState,
};

pub fn systems_router() -> Router<AppState> {
    Router::new().route("/v1/systems/ping", get(get_ping))
}

pub fn todos_router() -> Router<AppState> {
    Router::new().route("/v1/todos", get(get_todos))
}
