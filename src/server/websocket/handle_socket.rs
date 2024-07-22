use axum::extract::ws::{Message, WebSocket};
use std::net::SocketAddr;
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::{from_str, to_string};
use tokio::sync::broadcast;
use crate::lobby::Lobby;
use crate::server::AppState;
use crate::server::websocket::{WSBody, WSRequestType};
use crate::server::websocket::client_request::{ClientRequest, ClientRequestType};
use crate::server::websocket::websocket_service::{
    ws_create_lobby, ws_join_lobby, ws_leave_lobby
};

pub async fn handle_player_join(
    app_state: &Arc<AppState>,
    request: WSBody
) -> Result<(Lobby, broadcast::Sender<String>), String> {
    Ok(match request.req_type {
        WSRequestType::LobbyJoin => {
            ws_join_lobby(&app_state, request.clone()).await?
        },
        WSRequestType::LobbyCreate => {
            ws_create_lobby(&app_state, request.clone()).await?
        },
        _ => {
            return Err("Lobby connection error: wrong request type".to_string())
        }
    })
}

pub async fn handle_socket(mut socket: WebSocket, who: SocketAddr, app_state: Arc<AppState>) {
    // start connection handler (join or create lobby)
    let (mut sender, mut receiver) = socket.split();
    let mut tx = None::<broadcast::Sender<String>>;

    // current player id, generated during the first client query (join or create lobby)
    let mut current_player_id = None::<String>;

    // current lobby id, generated during the first client query (join or create lobby)
    let mut current_lobby_id = None::<String>;

    // container for message to client generated during message from client handling
    let mut raw_response = String::new();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(content) = msg {
            let request: WSBody = match from_str(&content) {
                Ok(req) => req,
                Err(err) => {
                    println!("Connection request parsing error: {}", err);
                    let _ = sender.send(Message::from("Failed to connect to lobby!")).await;
                    break
                }
            };

            let req_type = request.clone().req_type;

            if req_type == WSRequestType::LobbyCreate || req_type == WSRequestType::LobbyJoin {
                current_player_id = Some(request.sender_id.clone());

                let (result, new_tx) = match handle_player_join(&app_state, request).await {
                    Ok(res) => res,
                    Err(err) => {
                        println!("Lobby connection error: {}", err);
                        let _ = sender.send(Message::from("Failed to connect to lobby!")).await;
                        break;
                    }
                };

                println!("{:#?}", result);

                current_lobby_id = Some(result.get_id().to_string());

                tx = Some(new_tx.clone());
                raw_response = result.get_id().to_string();
                break;
            }

        } else {
            break;
        }
    }

    // websocket state handler
    if tx.is_some() {
        let tx = tx.unwrap();
        let mut rx = tx.subscribe();

        let _ = tx.send("player joined the lobby".to_string());

        let mut recv_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                println!("{raw_response}");
                let req_to_client = ClientRequest::new(
                    ClientRequestType::LobbyUpdate,
                    to_string(&raw_response).unwrap()
                );
                if sender.send(req_to_client.to_msg()).await.is_err() {
                    break;
                }
            }
        });

        let mut send_task = {
            let tx = tx.clone();
            tokio::spawn(async move {
                while let Some(Ok(Message::Text(text))) = receiver.next().await {
                    let _ = tx.send("test".to_string());
                }
            })
        };

        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        }

        let _ = tx.send("player left the lobby".to_string());
    }

    // If any one of the tasks exit, abort the other.

    // returning from the handler closes the websocket connection
    if current_lobby_id.is_some() && current_player_id.is_some() {
        match ws_leave_lobby(&app_state, &current_lobby_id.unwrap(), &current_player_id.unwrap()).await {
            Ok(_) => (),
            Err(err) => {
                println!("Error occurred while leaving lobby by player: {}", err);
            }
        }
    } else {
        println!("Websocket connection closing error:");
    }

    println!("Websocket context {who} destroyed");
}
