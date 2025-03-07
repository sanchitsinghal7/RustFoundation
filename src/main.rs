/*#![allow(unused_variables)]
#![allow(dead_code)]

mod foundation; // Import the foundation module
mod apis;

use std::net::SocketAddr;
use axum::serve; // ✅ Use `axum::serve` instead of `hyper::Server`
use tokio::net::TcpListener;
use apis::task_routes::create_router;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    /*foundation::basic_types_and_vars::boolean_demo();
    foundation::basic_types_and_vars::unsigned_integers_demo();
    foundation::basic_types_and_vars::signed_integers_demo();
    foundation::basic_types_and_vars::platform_specific_and_string_types_demo();
    foundation::collection_types_and_vars::tuple_demo();
    foundation::collection_types_and_vars::array_demo();
    foundation::collection_types_and_vars::slice_demo();
    foundation::collection_types_and_vars::hashmap_demo();
    foundation::object_types_and_vars::struct_ownership_demo();*/

    // ✅ Await the function to get a Router instance
    let app = create_router().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8011));
    println!("API Server running at http://{}", addr);

    // ✅ Use `tokio::net::TcpListener` to bind to the port
    let listener = TcpListener::bind(addr).await.unwrap();

    // ✅ Use `axum::serve()` instead of `Server::bind()`
    serve(listener, app.into_make_service()).await.unwrap();
}*/

use axum::{
    routing::post,
    Router,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber;

mod apis;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Initialize database pool
    let pool = init_db_pool().await;
    let app = apis::task_routes::create_router(pool);

    // Define server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server running at http://{}", addr);

    // Create a TcpListener
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");

    // Start the server using axum::serve
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error!("Server error: {}", e);
    }
}

async fn init_db_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (e.g., 'postgres://user:pass@localhost/task_db')");

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}