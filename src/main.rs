use anyhow::Result;
use crate::service::initialize_service;
use crate::web::run;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize service with NATS support
    let service = initialize_service().await?;
    
    // Run the web server with WebSocket and TLS support
    run(service).await?;
    
    Ok(())
}
