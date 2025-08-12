mod models;
mod state;
mod handlers;
mod routes;

use state::Db;

#[tokio::main]
async fn main() {
    let db: Db = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let app = routes::create_routes(db);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}