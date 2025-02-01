// src/domain/user.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::DateTime;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub created_at: DateTime,
    #[serde(skip_serializing)]
    pub metadata: HashMap<String, String>,
}
