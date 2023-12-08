use chrono::{NaiveDate, NaiveTime};
use diesel::OptionalExtension;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub url_slug: String,
    pub created_at: String, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: String, // Change to DateTime type based on your needs (e.g., chrono)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorkspace {
    pub user_id: i32,
    pub name: String,
    pub url_slug: String,
    pub created_at: String, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: String, // Change to DateTime type based on your needs (e.g., chrono)
}
