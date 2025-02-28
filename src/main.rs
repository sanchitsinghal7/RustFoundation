#![allow(unused_variables)]
#![allow(dead_code)]

mod foundation; // Import the foundation module
mod apis;

use std::net::SocketAddr;
use axum::serve; // ✅ Use `axum::serve` instead of `hyper::Server`
use tokio::net::TcpListener;
use apis::routes::create_router;

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
}