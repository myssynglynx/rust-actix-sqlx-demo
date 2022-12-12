use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct InputUser {
    pub username: String,
    pub address: String,
}

#[derive(Serialize, FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    address: String,
    joined_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct InputPost{
    pub title: String,
    pub content: String,
}

#[derive(Serialize, FromRow)]
pub struct Post {
    id: Uuid,
    author_id: Uuid,
    title: String,
    content: String,
    created_at: NaiveDateTime,
    edited_at: NaiveDateTime,
}
