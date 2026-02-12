use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category: String,
    pub max_recruits: i32,
    pub current_recruits: i32,
    pub is_finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub author_id: i32,
    pub attachments: Vec<String>,
    pub apply: i32,
    pub is_new: bool,
}
