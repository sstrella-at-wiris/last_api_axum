use crate::model::{User, UserForCreate};
use crate::repositories::users_repository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub async fn create_user(
    State(pool): State<Arc<Pool<Sqlite>>>,
    Json(user_fc): Json<UserForCreate>,
) -> Result<Json<User>, StatusCode> {
    let user = users_repository::insert_user(&pool, &user_fc)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

pub async fn list_users(State(pool): State<Arc<Pool<Sqlite>>>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = users_repository::get_all_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn delete_user(
    State(pool): State<Arc<Pool<Sqlite>>>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    let user = users_repository::get_user_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    users_repository::delete_user_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

pub async fn update_user(
    State(pool): State<Arc<Pool<Sqlite>>>,
    Path(id): Path<i64>,
    Json(user_fc): Json<UserForCreate>,
) -> Result<Json<User>, StatusCode> {
    let user = users_repository::update_user_by_id(&pool, id, &user_fc)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}