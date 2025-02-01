// src/main.rs
use actix_web::web;
use anyhow::Result;
use std::sync::Arc;
use crate::service::{UserService};
use crate::web::run;

#[actix_web::main]
async fn main() -> Result<()> {
    let service = Arc::new(UserService::new());
    let state = crate::web::State { service };

    run(state).await
}
