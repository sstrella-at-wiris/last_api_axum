mod db;
mod model;
mod routes;

use axum::{serve, Router};
use db::init_db;
use routes::user_routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_pool = init_db().await.expect("Failed to initialize DB");

    let app = Router::new()
        .nest("/users", user_routes(db_pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);

    serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
