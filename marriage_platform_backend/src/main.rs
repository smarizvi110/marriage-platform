use axum::{
    routing::{get, post},
    Router,
    Extension,
};
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use sqlx::{Pool, Postgres};
mod config; // Import the config module;
mod handlers; // Import the handlers module
mod services; // Import the services module
mod auth; // Import the auth module
mod db; // Import the db module
mod error; // Import the error module

#[tokio::main]
async fn main() {
    // Initialize database pool
    let pool: Pool<Postgres> = config::get_database_pool().await;

    // Build our application with routes and handlers
    let app = Router::new()
        .route("/", get(root))
        .route("/signup", post(handlers::user_handler::register_user_handler))
        .route("/login", post(handlers::user_handler::login_user_handler))
        .layer(TraceLayer::new_for_http()) // Logging and tracing layer
        .layer(Extension(pool)); // Pass pool as an extension to all handlers

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to the Marriage platform"
}
