pub mod handle_socket;
pub mod process_message;
pub mod message_body_handler;
use axum::{
    extract::ws::{WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use std::net::SocketAddr;
use axum::extract::connect_info::ConnectInfo;
use crate::server::websocket::handle_socket::handle_socket;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("Websocket connection established.");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}
