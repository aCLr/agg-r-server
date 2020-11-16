use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct RecordWithMeta {
    pub id: i32,
    pub title: Option<String>,
    pub guid: String,
    pub source_id: i32,
    pub content: String,
    pub date: NaiveDateTime,
    pub image: Option<String>,
    pub starred: Option<bool>,
}
