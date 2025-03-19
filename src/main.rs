use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use reqwest::Method;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

pub mod config;
pub mod handlers;
pub mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Load configuration
    let config = config::load_config();
    info!("Configuration loaded successfully");

    // Enhanced CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    info!("Setting up routes");

    // Build the application router
    let app = Router::new()
        .route("/users", post(handlers::sign_up_user))
        .route(
            "/users/{user_id}",
            get(handlers::get_user_data).put(handlers::update_user_data),
        )
        .layer(cors)
        .with_state(config); // Clone the Arc<AppState> to avoid ownership issues

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 1], 3000));
    info!("Server starting on {}", addr);

    // Use tokio's TcpListener instead of std's
    let listener = TcpListener::bind(addr).await?;
    info!("Server listening on {}", addr);

    // await the serve call instead of using the ? operator
    axum::serve(listener, app).await?;

    Ok(())
}
