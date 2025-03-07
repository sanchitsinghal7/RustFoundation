use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Error as SqlxError};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error};

// Task creation request struct
#[derive(Debug, Deserialize, Clone)]
struct TaskCreate {
    task_name: String,
    description: Option<String>,
    status: String,
    due_date: DateTime<Utc>,
    assignees: Option<Vec<String>>, // List of emails
    task_file_ids: Option<Vec<i32>>,
}

// Task response struct
#[derive(Debug, Serialize)]
struct TaskResponse {
    task_id: i32,
    task_name: String,
    status: String,
    due_date: String, // ISO format
    assigned_users: Vec<String>,
    message: String,
}

// App state to hold the database pool
#[derive(Clone)]
struct AppState {
    db_pool: Arc<PgPool>,
}

// Mock functions (replace with actual implementations)
async fn get_user_id_by_email(_pool: &PgPool, email: &str) -> Option<i32> {
    // Mock: Return a user ID or None
    if email == "test@example.com" { Some(2) } else { None }
}

async fn assign_task_to_users(_pool: &PgPool, task_id: i32, assignees_ids: Vec<i32>, _user_id: i32) {
    info!("Assigned task {} to users {:?}", task_id, assignees_ids);
}

async fn update_task_files_with_task_id(_pool: &PgPool, task_file_ids: Vec<i32>, task_id: i32) {
    info!("Updated task files {:?} with task_id {}", task_file_ids, task_id);
}

// Database CRUD operation
async fn create_task_db(pool: &PgPool, task: TaskCreate, user_id: i32) -> Result<i32, SqlxError> {
    let now = Utc::now();
    let task = sqlx::query_as::<_, (i32,)>(  // Return just the ID
        r#"
        INSERT INTO task_manager.tasksrust (
            task_name, description, status, due_date, created_by, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(&task.task_name)
    .bind(&task.description)
    .bind(&task.status)
    .bind(task.due_date)
    .bind(user_id)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    info!("✅ Task Created: {}", task.0);
    Ok(task.0)
}

// Controller logic
async fn create_task_controller(
    pool: &PgPool,
    task: TaskCreate,
    user_id: i32,
) -> Result<TaskResponse, String> {
    let task_id = create_task_db(pool, task.clone(), user_id)
        .await
        .map_err(|e| format!("Error creating task: {}", e))?;

    let mut assignees_ids = Vec::new();
    if let Some(assignees) = &task.assignees {
        for email in assignees {
            match get_user_id_by_email(pool, email).await {
                Some(id) => assignees_ids.push(id),
                None => warn!("No user ID found for email: {}", email),
            }
        }
    }

    if !assignees_ids.is_empty() {
        assign_task_to_users(pool, task_id, assignees_ids.clone(), user_id)
            .await;
    }

    if let Some(task_file_ids) = &task.task_file_ids {
        update_task_files_with_task_id(pool, task_file_ids.clone(), task_id)
            .await;
    }

    Ok(TaskResponse {
        task_id,
        task_name: task.task_name,
        status: task.status,
        due_date: task.due_date.to_rfc3339(), // ISO format using chrono
        assigned_users: task.assignees.unwrap_or_default(),
        message: "Task created and assigned successfully".to_string(),
    })
}

// API handler
async fn create_task(
    State(state): State<AppState>,
    Json(task): Json<TaskCreate>,
) -> impl IntoResponse {
    let user_id = 1; // Mock user ID, replace with real auth

    match create_task_controller(&state.db_pool, task, user_id).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(e) => {
            error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e})),
            )
                .into_response()
        }
    }
}

pub fn create_router(pool: PgPool) -> Router {
    let state = AppState {
        db_pool: Arc::new(pool),
    };
    Router::new()
        .route("/tasks", post(create_task))
        .with_state(state)
}