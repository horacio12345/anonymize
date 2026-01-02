// src/main.rs

use anonymize::web::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Port configurable via environment variable
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Start web server
    start_server(port).await?;

    Ok(())
}
