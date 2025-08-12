use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use crate::{models::{Item, ItemCreate, ItemUpdate}, state::Db};

pub async fn create_item(
    State(db): State<Db>,
    Json(payload): Json<ItemCreate>,
) -> impl IntoResponse {
    let mut items = db.lock().await;
    let item = Item {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
    };
    items.push(item.clone());
    (StatusCode::CREATED, Json(item))
}

pub async fn list_items(State(db): State<Db>) -> impl IntoResponse {
    let items = db.lock().await;
    Json(items.clone())
}

pub async fn get_item(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
    let items = db.lock().await;
    if let Some(item) = items.iter().find(|item| item.id == id) {
        Json(item.clone()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn update_item(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
    Json(payload): Json<ItemUpdate>,
) -> impl IntoResponse {
    let mut items = db.lock().await;
    if let Some(item) = items.iter_mut().find(|item| item.id == id) {
        if let Some(name) = payload.name {
            item.name = name;
        }
        if let Some(description) = payload.description {
            item.description = description;
        }
        return (StatusCode::OK, Json(item.clone())).into_response();
    }
    StatusCode::NOT_FOUND.into_response()
}

pub async fn delete_item(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
    let mut items = db.lock().await;
    let initial_len = items.len();
    items.retain(|item| item.id != id);
    if items.len() < initial_len {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
