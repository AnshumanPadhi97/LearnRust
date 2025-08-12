use axum::{
    routing::{get},
    Router,
};
use crate::{handlers::*, state::Db};

pub fn create_routes(db: Db) -> Router {
    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/{id}", get(get_item).delete(delete_item).put(update_item))
        .with_state(db)
}
