use actix_web::{App, HttpServer, Responder, web};
use actix_service::ServiceExt;
use std::sync::Arc;
use anyhow::Result;
use serde_json::Value;
use tokio::net::TcpListener;
use openssl::ssl::{Ssl, SslMethod, SslAcceptor};
use tokio::io::{ReadHalf, WriteHalf};
use websocket::WebSocket;
use websocket::result::TcpStreamExt;
use std::net::SocketAddr;

/// WebSocket handler
async fn ws_handler(
    ws: WebSocket<(ReadHalf<TcpStream>, WriteHalf<TcpService>)>,
    service: Arc<UserService>,
) -> Result<()> {
    let (mut write, mut read) = ws.split();
    
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                // Process WebSocket message
                let data = String::from_utf8(msg.to_vec())?;
                let user: User = serde_json::from_str(&data)?;
                service.create_user(user).await?;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Run the web server with WebSocket and TLS support
pub async fn run(service: Arc<UserService>) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8443));
    let listener = TcpListener::bind(addr).await?;
    
    // Configure SSL context
    let mut ssl = Ssl::new(SslMethod::tls())?;
    ssl.set_certificate_file("cert.pem")?;
    ssl.set_private_key_file("key.pem", openssl::ssl::SSL_FILETYPE_PEM)?;
    
    let acceptor = SslAcceptor::new(ssl)?;
    let server = acceptor.build(listener)?.incoming();

    // Start the WebSocket server
    let mut incoming = server;
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let (read, write) = tokio::io::split(stream);
        let ws = WebSocket::from_parts(read, write);
        
        tokio::spawn(async move {
            ws_handler(ws, Arc::clone(&service)).await;
        });
    }

    Ok(())
}
