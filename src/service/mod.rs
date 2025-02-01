pub mod service;
use serde::Deserialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use crate::nats::start_nats_listener;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
}

pub struct UserService {
    pub users: Arc<Mutex<VecDeque<User>>>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        Ok(self.users.lock().await.clone().into_iter().collect())
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        let mut users = self.users.lock().await;
        users.push_back(user.clone());
        Ok(user)
    }
}

// Initialize the service with NATS support
pub async fn initialize_service() -> Result<Arc<UserService>> {
    let service = Arc::new(UserService::new());
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    
    // Start NATS listener in the background
    tokio::spawn(async move {
        if let Err(e) = start_nats_listener(rx, Arc::clone(&service)).await {
            eprintln!("NATS listener error: {}", e);
        }
    });

    Ok(service)
}
