use axum::Router;
use rest::route::systems_router;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(systems_router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
