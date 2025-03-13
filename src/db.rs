use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&db_url).await?;
    
    sqlx::migrate!().run(&pool).await?; // Runs migrations

    Ok(pool)
}
