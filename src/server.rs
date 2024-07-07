pub mod controllers;
pub mod redis_service;
pub mod errors;

use dotenv;
use redis::AsyncCommands;
use axum::{routing::get, Router};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use crate::server::controllers::lobby_controller::{create_lobby, delete_lobby, get_lobby, get_lobby_by_id};

#[derive(Clone)]
pub struct AppState {
    redis_pool: Pool<RedisConnectionManager>
}

pub async fn create_app(redis_pool: Pool<RedisConnectionManager>) {
    let state = AppState{ redis_pool };

    let app = Router::new()
        .fallback(fallback)
        .route(
            "/lobby",
                get(get_lobby)
                .post(create_lobby)

        )
        .route(
             "/lobby/:id",
             get(get_lobby_by_id)
                 .delete(delete_lobby)
         )
        .with_state(state);

    let port = dotenv::var("SERVER_PORT")
        .expect("SERVER_PORT environment variable not defined.");

    let host = dotenv::var("SERVER_HOST")
        .expect("SERVER_HOST environment variable not defined.");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap();
    println!("Serving at http://{}:{}", host, port);

    axum::serve(listener, app).await.unwrap();
}

pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri)
    )
}
