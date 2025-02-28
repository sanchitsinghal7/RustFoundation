use axum::{extract::{Path, State}, routing::{get, post, put, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use uuid::Uuid;

// Define a struct to hold application state (Database Connection Pool)
#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

// User model for JSON serialization/deserialization
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}

// Handler to create a new user (POST /users)
pub async fn create_user(State(state): State<AppState>, Json(user): Json<User>) -> Json<User> {
    let new_user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        user.name,
        user.email
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to insert user");

    Json(new_user)
}

// Handler to get all users (GET /users)
pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&state.db)
        .await
        .expect("Failed to fetch users");

    Json(users)
}

// Handler to get a single user by ID (GET /users/:id)
pub async fn get_user(Path(id): Path<Uuid>, State(state): State<AppState>) -> Json<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        id
    )
    .fetch_one(&state.db)
    .await
    .expect("User not found");

    Json(user)
}

// Handler to update a user (PUT /users/:id)
pub async fn update_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Json<User> {
    let updated_user = sqlx::query_as!(
        User,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email",
        user.name,
        user.email,
        id
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to update user");

    Json(updated_user)
}

// Handler to delete a user (DELETE /users/:id)
pub async fn delete_user(Path(id): Path<Uuid>, State(state): State<AppState>) -> Json<String> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&state.db)
        .await
        .expect("Failed to delete user");

    Json(format!("User {} deleted", id))
}

// ✅ Function to create the router (for `main.rs` to use)
pub async fn create_router() -> Router {
    dotenv::dotenv().ok();

    // Connect to the PostgreSQL database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let state = AppState { db: db_pool };

    Router::new()
    .route("/users", post(create_user).get(get_users))
    .route(
        "/users/:id",
        get(get_user)
            .put(update_user)  // Ensure the handler is properly referenced
            .delete(delete_user), // Ensure delete is properly referenced
    )
    .with_state(state)
    
}