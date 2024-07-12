pub mod controllers;
pub mod routes;
pub mod redis_service;
pub mod errors;
mod websocket;

use std::net::SocketAddr;
use dotenv;
use redis::AsyncCommands;
use axum::{routing::get, routing::patch, routing::post, Router};
use axum::http::Method;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use tower_http::cors::{Any, CorsLayer};
use crate::server::routes::lobby_routes::{
    route_add_player_to_lobby,
    route_create_lobby,
    route_delete_lobby,
    route_delete_player_from_lobby,
    route_get_lobbies,
    route_get_lobby_by_id
};
use crate::server::routes::player_routes::{
    route_create_player,
    route_get_player_by_id
};
use crate::server::websocket::websocket_handler;

#[derive(Clone)]
pub struct AppState {
    redis_pool: Pool<RedisConnectionManager>
}

pub async fn create_app(redis_pool: Pool<RedisConnectionManager>) {
    let state = AppState{
        redis_pool
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET]);

    let app = Router::new()
        .fallback(fallback)
        // WEBSOCKET
        .route("/ws", get(websocket_handler))
        // LOBBY
        .route(
            "/lobby",
                get(route_get_lobbies)
                .post(route_create_lobby)

        )
        .route(
             "/lobby/:id",
             get(route_get_lobby_by_id)
                 .delete(route_delete_lobby)
        )
        .route(
            "/lobby/:lobby_id/:player_id",
            patch(route_add_player_to_lobby)
                .delete(route_delete_player_from_lobby)
        )
        // PLAYER
        .route(
            "/player",
            post(route_create_player)
        )
        .route(
            "/player/:id",
            get(route_get_player_by_id)
        )
        .with_state(state)
        .layer(cors);

    let port = dotenv::var("SERVER_PORT")
        .expect("SERVER_PORT environment variable not defined.");

    let host = dotenv::var("SERVER_HOST")
        .expect("SERVER_HOST environment variable not defined.");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await.unwrap();
    println!("Serving at http://{}:{}", host, port);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await.unwrap();
}

pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri)
    )
}
