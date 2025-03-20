use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub id: i64,
    pub key: String,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct LicenseForCreate {
    pub key: String,
    pub user_id: i64,
}