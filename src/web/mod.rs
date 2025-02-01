// src/web/mod.rs
use actix_web::{App, HttpServer, Responder, web};
use actix_service::ServiceExt;
use std::sync::Arc;
use anyhow::Result;
use crate::service::UserService;

#[derive(Clone)]
pub struct State {
    pub service: Arc<UserService>,
}

pub async fn run(state: State) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .state(state.clone())
            .service(web::resource("/api/users").route(web::get().to(get_users)))
            .service(web::resource("/api/users").route(web::post().to(create_user)))
    })
    .bind("127.0.0.1:8080")
    .expect("Cannot bind to port 8080")
    .run()
    .await?;
    Ok(())
}

async fn get_users(web::State(service): web::State<Arc<UserService>>) -> impl Responder {
    let users = service.get_users().await.unwrap();
    actix_web:: HttpResponse::Ok().json(users)
}

async fn create_user(
    web::State(service): web::State<Arc<UserService>>,
    web::Json(user): web::Json<User>,
) -> impl Responder {
    let user = service.create_user(user).await.unwrap();
    actix_web::HttpResponse::Created().json(user)
}
