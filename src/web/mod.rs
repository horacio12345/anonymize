// src/web/mod.rs

pub mod handlers;

use axum::{
    routing::post,
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use std::net::SocketAddr;

/// Create the main application router
pub fn create_router() -> Router {
    // CORS for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // API endpoints
        .route("/api/anonymize", post(handlers::anonymize_handler))
        .route("/api/anonymize-file", post(handlers::anonymize_file_handler))
        // Serve static files
        .nest_service("/", ServeDir::new("src/web/static"))
        .layer(cors)
}

/// Start the web server
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("🚀 Server started at http://0.0.0.0:{}", port);
    println!("   Open http://localhost:{} in your browser", port);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
