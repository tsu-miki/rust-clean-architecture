use axum::Router;
use rest::{
    route::{systems_router, todos_router},
    state::create_app_state,
};

#[tokio::main]
async fn main() {
    let app_state = create_app_state();

    let app = Router::new()
        .merge(systems_router())
        .merge(todos_router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
