use axum::{routing::get, Router};
use handler::systems;

pub fn systems_router() -> Router {
    Router::new().route("/v1/systems/ping", get(systems::ping))
}

