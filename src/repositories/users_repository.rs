use crate::model::{User, UserForCreate};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub async fn insert_user(pool: &Arc<Pool<Sqlite>>, user_fc: &UserForCreate) -> Result<User, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        user_fc.name,
        user_fc.email
    )
    .execute(&**pool)
    .await?;

    Ok(User {
        id: result.last_insert_rowid(),
        name: user_fc.name.clone(),
        email: user_fc.email.clone(),
    })
}

pub async fn get_all_users(pool: &Arc<Pool<Sqlite>>) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&**pool)
        .await?;

    Ok(users)
}

pub async fn get_user_by_id(pool: &Arc<Pool<Sqlite>>, id: i64) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = ?", id)
        .fetch_one(&**pool)
        .await?;

    Ok(user)
}

pub async fn delete_user_by_id(pool: &Arc<Pool<Sqlite>>, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&**pool)
        .await?;

    Ok(())
}

pub async fn update_user_by_id(pool: &Arc<Pool<Sqlite>>, id: i64, user_fc: &UserForCreate) -> Result<User, sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET name = ?, email = ? WHERE id = ?",
        user_fc.name,
        user_fc.email,
        id
    )
    .execute(&**pool)
    .await?;

    Ok(User {
        id,
        name: user_fc.name.clone(),
        email: user_fc.email.clone(),
    })
}