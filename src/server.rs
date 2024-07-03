pub mod controllers;
pub mod errors;

use std::collections::HashMap;
use std::sync::Arc;
use axum::{routing::get, Router};
use tower_http::add_extension::AddExtensionLayer;
use tokio::sync::RwLock;
use crate::lobby::Lobby;
use crate::server::controllers::lobby_controller::{
    create_lobby,
    delete_lobby,
    get_lobby,
    get_lobby_by_id
};

#[derive(Debug, Clone)]
pub struct AppState {
    game_data: GameData
}

#[derive(Clone, Debug)]
struct GameData {
    lobby_pool: HashMap<u64, Lobby>
}

pub async fn create_app(lobby_pool: HashMap<u64, Lobby>) {
    let mut state = Arc::new(RwLock::new(AppState{ game_data: GameData{lobby_pool} }));

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
        .layer(AddExtensionLayer::new(state));

    let listener = tokio::net::TcpListener::bind("localhost:3500").await.unwrap();
    println!("Serving on http://localhost:3500");

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
