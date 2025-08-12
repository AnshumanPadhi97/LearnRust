use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemCreate {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}