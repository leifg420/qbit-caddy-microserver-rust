use nats::Connection;
use anyhow::Result;
use tokio::sync::mpsc;
use serde_json::Value;

/// Initialize NATS connection
async fn connect_nats() -> Result<Connection> {
    let nc = nats::connect("nats://localhost:4222").await?;
    Ok(nc)
}

/// NATS listener task
pub async fn start_nats_listener(
    mut rx: mpsc::Receiver<Value>,
    service: std::sync::Arc<crate::service::UserService>,
) -> Result<()> {
    let nc = connect_nats().await?;
    let sub = nc.subscribe("user_updates").await?;
    
    tokio::spawn(async move {
        while let Ok(msg) = sub.next().await {
            let payload = msg.bytes()?;
            // Process the message and update the service
            let user = serde_json::from_slice(&payload)?;
            service.create_user(user).await?;
        }
        Ok::<(), anyhow::Error>(())
    });

    Ok(())
}
