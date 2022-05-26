use bson;
use chrono;
use serde::{Deserialize, Serialize};

pub type Timestamp = bson::DateTime;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name {
    pub complete: String,
    pub title: String,
    pub prefix: String,
    pub first: String,
    pub middle: String,
    pub last: String,
    pub suffix: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: bson::oid::ObjectId,
    pub username: String,
    pub created_at: Timestamp,
    pub modified_at: Timestamp,
}

impl User {
    pub fn new(username: String) -> Self {
        let id = bson::oid::ObjectId::new();
        let created_at: Timestamp = chrono::Local::now().into();
        Self {
            id,
            username,
            created_at,
            modified_at: created_at,
        }
    }
}