pub mod controllers;
pub mod redis_service;
pub mod errors;
mod websocket;

use dotenv;
use redis::AsyncCommands;
use axum::{routing::get, routing::patch, routing::post, Router};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use crate::server::controllers::lobby_controller::{
    add_player_to_lobby,
    create_lobby,
    delete_lobby,
    delete_player_from_lobby,
    get_lobbies,
    get_lobby_by_id
};
use crate::server::controllers::player_controller::{
    create_player,
    get_player_by_id
};
use crate::server::websocket::websocket_handler;

#[derive(Clone)]
pub struct AppState {
    redis_pool: Pool<RedisConnectionManager>
}

pub async fn create_app(redis_pool: Pool<RedisConnectionManager>) {
    let state = AppState{ redis_pool };

    let app = Router::new()
        .fallback(fallback)
        // WEBSOCKET
        .route("/ws", get(websocket_handler))
        // LOBBY
        .route(
            "/lobby",
                get(get_lobbies)
                .post(create_lobby)

        )
        .route(
             "/lobby/:id",
             get(get_lobby_by_id)
                 .delete(delete_lobby)
         )
        .route(
            "/lobby/:lobby_id/:player_id",
            patch(add_player_to_lobby)
                .delete(delete_player_from_lobby)
        )
        // PLAYER
        .route(
            "/player",
            post(create_player)
        )
        .route(
            "/player/:id",
            get(get_player_by_id)
        )
        .with_state(state);

    let port = dotenv::var("SERVER_PORT")
        .expect("SERVER_PORT environment variable not defined.");

    let host = dotenv::var("SERVER_HOST")
        .expect("SERVER_HOST environment variable not defined.");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await.unwrap();
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
