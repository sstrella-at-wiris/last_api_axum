use crate::model::{User, UserForCreate};
use axum::{extract::{State, Path}, http::StatusCode, routing::*, Json, Router};
use sqlx::{Pool, Sqlite};

pub fn user_routes(db_pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/list", get(list_users))
        .route("/:id", delete(delete_user))
        .with_state(db_pool)
}

async fn create_user(
    State(pool): State<Pool<Sqlite>>,
    Json(user_fc): Json<UserForCreate>,
) -> Result<Json<User>, StatusCode> {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        user_fc.name,
        user_fc.email
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = User {
        id: result.last_insert_rowid(),
        name: user_fc.name,
        email: user_fc.email,
    };

    Ok(Json(user))
}

async fn list_users(State(pool): State<Pool<Sqlite>>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

async fn delete_user(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = ?", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
