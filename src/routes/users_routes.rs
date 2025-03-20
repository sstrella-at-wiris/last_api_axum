use crate::services::users_service::{create_user, list_users, delete_user, update_user};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use axum::{

    routing::{delete, get, patch, post},
    Router,
};


pub fn users_routes(db_pool: Arc<Pool<Sqlite>>) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/list", get(list_users))
        .route("/:id", delete(delete_user))
        .route("/:id", patch(update_user))
        .with_state(Arc::clone(&db_pool))
}

