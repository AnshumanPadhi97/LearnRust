use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::Item;

pub type Db = Arc<Mutex<Vec<Item>>>;