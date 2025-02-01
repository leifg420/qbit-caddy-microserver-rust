// src/service/mod.rs
use crate::domain::user::{User};
use std::collections::VecDeque;
use anyhow::Result;
use actix_service::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::FutureExt;

#[derive(Clone)]
pub struct UserService {
    users: VecDeque<User>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: VecDeque::new(),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        Ok(self.users.clone().into_iter().collect())
    }

    pub async fn create_user(&mut self, user: User) -> Result<User> {
        self.users.push_back(user.clone());
        Ok(user)
    }
}

impl Transform<ServiceRequest, ServiceResponse> for UserService {
    type Transform = Self;

    fn transform(&self, req: ServiceRequest) -> Self::Transform {
        self.clone()
    }
}
