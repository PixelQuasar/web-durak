use crate::lobby::PopulatedLobby;
use crate::server::websocket::client_request::{ClientRequest, ClientRequestType};
use crate::server::websocket::process_message::{
    disconnect_message, handle_message, handle_player_join,
};
use crate::server::websocket::{WSBody, WSError, WSRequestType};
use crate::server::AppState;
use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::{from_str, to_string};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;

pub async fn handle_socket(socket: WebSocket, who: SocketAddr, app_state: Arc<AppState>) {
    // start connection handler (join or create lobby)
    let (mut sender, mut receiver) = socket.split();

    let mut tx = None::<broadcast::Sender<String>>;

    // current player id, generated during the first client query (join or create lobby)
    let mut current_player_id = None::<String>;

    // current lobby id, generated during the first client query (join or create lobby)
    let mut current_lobby_id = None::<String>;

    // container for message to client generated during first message from client handling
    let mut first_response = String::new();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(content) = msg {
            let request = match from_str::<WSBody>(&content) {
                Ok(req) => req,
                Err(err) => {
                    println!("Connection request parsing error: {}", err);
                    let _ = sender
                        .send(Message::from(
                            WSError::conn_error("Lobby connection error".to_string()).stringify(),
                        ))
                        .await;
                    break;
                }
            };

            let req_type = request.clone().req_type;

            if req_type == WSRequestType::LobbyCreate || req_type == WSRequestType::LobbyJoin {
                current_player_id = Some(request.sender_id.clone());

                let (result, new_tx) = match handle_player_join(&app_state, request).await {
                    Ok(res) => res,
                    Err(err) => {
                        println!("Lobby connection error: {}", err);
                        let _ = sender
                            .send(Message::from(WSError::conn_error(err).stringify()))
                            .await;
                        break;
                    }
                };

                current_lobby_id = Some(result.get_id().to_string());

                tx = Some(new_tx.clone());
                first_response = to_string::<PopulatedLobby>(&result).unwrap();
            }
        }
        break;
    }

    // websocket state handler
    if tx.is_some() {
        let tx = tx.unwrap();
        let mut rx = tx.subscribe();

        let _ =
            tx.send(ClientRequest::new(ClientRequestType::LobbyUpdate, first_response).to_string());

        let mut recv_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });

        let mut send_task = {
            let tx = tx.clone();

            let app_state = app_state.clone();

            tokio::spawn(async move {
                while let Some(Ok(Message::Text(text))) = receiver.next().await {
                    let request = match from_str::<WSBody>(&text) {
                        Ok(res) => res,
                        Err(err) => {
                            println!("message processing error: {}", err);
                            break;
                        }
                    };

                    let (res_type, response) = handle_message(&app_state, request)
                        .await
                        .unwrap_or_else(|err| {
                            (
                                ClientRequestType::Error,
                                WSError::game_error(err).stringify(),
                            )
                        });

                    let req_to_client = ClientRequest::new(res_type, response);

                    let _ = tx.send(req_to_client.to_string());
                }
            })
        };

        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        }

        match disconnect_message(&app_state, current_lobby_id, current_player_id).await {
            Ok(res) => {
                let _ = tx.send(res);
            }
            Err(err) => {
                println!("Disconnection error: {}", err);
                let _ = tx.send(
                    ClientRequest::build_error(WSError::game_error(err).stringify()).to_string(),
                );
            }
        }
    }

    println!("Websocket context {who} destroyed");
}
