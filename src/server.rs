pub mod controllers;
pub mod errors;
pub mod redis_service;
pub mod routes;
mod websocket;

use crate::server::routes::lobby_routes::{
    route_create_lobby, route_delete_lobby, route_get_lobbies, route_get_lobby_by_id,
    route_get_lobby_score_board,
};
use crate::server::routes::player_routes::{route_create_player, route_get_player_by_id};
use crate::server::websocket::websocket_handler;
use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::{routing::get, routing::patch, routing::post, Router};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use dotenv;
use redis::AsyncCommands;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::routing::MethodRouter;
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct LobbyConnection {
    tx: broadcast::Sender<String>,
}

impl LobbyConnection {
    pub fn new() -> LobbyConnection {
        LobbyConnection {
            tx: broadcast::channel(64).0,
        }
    }
}

pub struct AppState {
    redis_pool: Pool<RedisConnectionManager>,
    lobby_connections: Mutex<HashMap<String, LobbyConnection>>,
}

pub async fn create_app(redis_pool: Pool<RedisConnectionManager>) {
    let state = Arc::new(AppState {
        redis_pool,
        lobby_connections: Mutex::new(HashMap::new()),
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let api_prefix = dotenv::var("API_PREFIX").expect("SERVER_PORT environment variable not defined.");

    let app = Router::new()
        .route(&api_prefix, MethodRouter::new()
            .fallback(fallback)
            // WEBSOCKET
            .route("/ws", get(websocket_handler))
            // LOBBY
            .route("/lobby", get(route_get_lobbies).post(route_create_lobby))
            .route(
                "/lobby/:id",
                get(route_get_lobby_by_id).delete(route_delete_lobby),
            )
            .route("/lobby/scoreboard/:id", get(route_get_lobby_score_board))
            // PLAYER
            .route("/player", post(route_create_player))
            .route("/player/:id", get(route_get_player_by_id))
            .with_state(state)
            .layer(cors)
        );

    let port = dotenv::var("SERVER_PORT").expect("SERVER_PORT environment variable not defined.");

    let host = dotenv::var("SERVER_HOST").expect("SERVER_HOST environment variable not defined.");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    println!("Serving at http://{}:{}", host, port);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}
