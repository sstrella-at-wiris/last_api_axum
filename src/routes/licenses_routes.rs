use crate::services::licenses_service::create_license;
use axum::{
    routing::post,
    Router,
};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub fn license_routes(db_pool: Arc<Pool<Sqlite>>) -> Router {
    Router::new()
        .with_state(Arc::clone(&db_pool))
}