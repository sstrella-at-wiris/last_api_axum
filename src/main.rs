mod db;
mod model;
mod routes;
mod services;
mod repositories;

use axum::{response::Html, serve, Router};
use axum::routing::*;
use db::init_db;
use routes::users_routes::users_routes;
use routes::licenses_routes::license_routes;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();
    let db_pool = Arc::new(init_db().await.expect("Failed to initialize DB"));

    let app = Router::new()
        .route("/hello", get(return_hello))
        .nest("/users", users_routes(db_pool.clone()))
        .nest("/licenses", license_routes(db_pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);

    serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app.into_make_service())
        .await
        .unwrap();
}

async fn return_hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}