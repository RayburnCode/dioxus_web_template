// backend/src/main.rs - Manual TLS approach
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use backend::config::{AppState, setup_database};
use backend::routes::api;
use std::env;
use migration::{Migrator, MigratorTrait};

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Create database connection
    let db_conn: sea_orm::DatabaseConnection = setup_database().await
        .expect("Failed to connect to database");

    // Run migrations automatically on startup
    println!("🔄 Running database migrations...");
    Migrator::up(&db_conn, None).await
        .expect("Failed to run migrations");
    println!("✅ Database migrations completed");

    // Create app state
    let app_state = AppState { db_conn };

    // Configure CORS origins from environment or use permissive default for development
    let cors_layer = match env::var("CORS_ORIGINS") {
        Ok(origins_str) => {
            let origins: Result<Vec<_>, _> = origins_str
                .split(',')
                .map(|s| s.trim().parse())
                .collect();
            
            match origins {
                Ok(parsed_origins) => {
                    println!("🔒 Using configured CORS origins: {}", origins_str);
                    CorsLayer::new().allow_origin(parsed_origins)
                }
                Err(e) => {
                    println!("⚠️  Failed to parse CORS_ORIGINS '{}': {}. Using permissive CORS for development.", origins_str, e);
                    CorsLayer::new().allow_origin(Any)
                }
            }
        }
        Err(_) => {
            println!("⚠️  CORS_ORIGINS not set. Using permissive CORS for development.");
            println!("   Set CORS_ORIGINS environment variable for production (e.g., 'https://example.com,https://app.example.com')");
            CorsLayer::new().allow_origin(Any)
        }
    };

    let app = api::routes()
        .with_state(app_state)
        .layer(
            cors_layer
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
                .allow_headers(Any) // More permissive for development
                .expose_headers(Any) // More permissive for development
                .allow_credentials(true),
        );


    // Get host and port from environment variables or use defaults
    let host = env::var("BACKEND_HOST")
        .unwrap_or_else(|_| {
            println!("⚠️  BACKEND_HOST not set, using default: 0.0.0.0");
            "0.0.0.0".to_string()
        });
    
    let port = env::var("BACKEND_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| {
            println!("⚠️  BACKEND_PORT not set or invalid, using default: 8083");
            8083
        });

    // Validate host format
    if !host.chars().all(|c| c.is_ascii_digit() || c == '.') && host != "localhost" {
        println!("❌ Invalid BACKEND_HOST format: '{}'. Expected format: '0.0.0.0' or 'localhost'", host);
    }

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Failed to parse host:port into SocketAddr");
    
    println!("🚀 Backend server starting...");
    println!("📍 Listening on: {}:{}", host, port);
    println!("🌐 Access URL: http://{}:{}", 
        if host == "0.0.0.0" { "localhost" } else { &host }, 
        port
    );
    println!("📋 Environment: {}", env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()));
    
    // Debug environment variables (without sensitive data)
    println!("🔍 Configuration Debug:");
    println!("  - BACKEND_HOST: {}", host);
    println!("  - BACKEND_PORT: {}", port);
    println!("  - DATABASE_URL: {}", 
        if env::var("DATABASE_URL").is_ok() { "✅ Set" } else { "❌ Missing" }
    );
    println!("  - CORS_ORIGINS: {}", 
        env::var("CORS_ORIGINS").unwrap_or_else(|_| "❌ Not set (using permissive CORS)".to_string())
    );
 
    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}