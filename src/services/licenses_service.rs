use crate::model::{License, LicenseForCreate};
use crate::repositories::licenses_repository;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub async fn create_license(
    State(pool): State<Arc<Pool<Sqlite>>>,
    Json(license_fc): Json<LicenseForCreate>,
) -> Result<Json<License>, StatusCode> {
    let license = licenses_repository::insert_license(&pool, &license_fc)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(license))
}