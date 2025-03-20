use crate::model::{License, LicenseForCreate};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub async fn insert_license(pool: &Arc<Pool<Sqlite>>, license_fc: &LicenseForCreate) -> Result<License, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO licenses (key, user_id) VALUES (?, ?)",
        license_fc.key,
        license_fc.user_id
    )
    .execute(&**pool)
    .await?;

    Ok(License {
        id: result.last_insert_rowid(),
        key: license_fc.key.clone(),
        user_id: license_fc.user_id.clone(),
    })
}