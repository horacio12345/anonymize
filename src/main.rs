// src/main.rs

use anonymize::web::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Puerto configurable vía variable de entorno
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT debe ser un número válido");

    // Iniciar servidor web
    start_server(port).await?;

    Ok(())
}
