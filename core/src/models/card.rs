use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Card {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub contents: serde_json::Value,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct CardCreate {
    pub author_id: Uuid,
    pub title: String,
    pub contents: Option<serde_json::Value>,
    pub tags: Vec<String>,
}
