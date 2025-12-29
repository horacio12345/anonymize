// src/web/mod.rs

pub mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use std::net::SocketAddr;

/// Crear el router principal de la aplicación
pub fn create_router() -> Router {
    // CORS para desarrollo
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // API endpoints
        .route("/api/anonymize", post(handlers::anonymize_handler))
        .route("/api/anonymize-file", post(handlers::anonymize_file_handler))
        // Servir archivos estáticos
        .nest_service("/", ServeDir::new("src/web/static"))
        .layer(cors)
}

/// Iniciar el servidor web
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("🚀 Servidor iniciado en http://0.0.0.0:{}", port);
    println!("   Abre http://localhost:{} en tu navegador", port);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
