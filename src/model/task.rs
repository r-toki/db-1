use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct NewTask {
    pub description: String,
}

#[derive(Serialize)]
pub struct EditTask {
    pub description: String,
    pub completed: bool,
}
