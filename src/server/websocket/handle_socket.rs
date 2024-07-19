use axum::extract::ws::{Message, WebSocket};
use std::net::SocketAddr;
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::from_str;
use tokio::sync::broadcast;
use crate::server::AppState;
use crate::server::websocket::{WSLobbyBody};
use crate::server::websocket::websocket_service::{ws_create_lobby, ws_join_lobby, ws_leave_lobby};

pub async fn handle_socket(mut socket: WebSocket, who: SocketAddr, app_state: Arc<AppState>) {
    // start connection handler (join or create lobby)
    let (mut sender, mut receiver) = socket.split();
    let mut tx = None::<broadcast::Sender<String>>;
    let mut client_game_id = None::<String>;
    let mut current_lobby_id = None::<String>;

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(content) = msg {
            println!("{:#?}", content);
            let request: WSLobbyBody = match from_str(&content) {
                Ok(req) => req,
                Err(err) => {
                    println!("Connection request parsing error: {}", err);
                    let _ = sender.send(Message::from("Failed to connect to lobby!")).await;
                    break
                }
            };
            client_game_id = Some(request.sender_id);
            current_lobby_id = request.lobby_id;
            match request.lobby_id.clone() {
                Some(lobby_id) => {
                    let (result, new_tx) = match ws_join_lobby(&app_state, request.clone()).await {
                        Ok(result) => result,
                        Err(err) => {
                            println!("Websocket message handling error: {}", err);
                            let _ = sender.send(Message::Text(err));
                            return;
                        }
                    };
                    tx = Some(new_tx);
                },
                None => {
                    let (result, new_tx) = match ws_create_lobby(&app_state, request.clone()).await {
                        Ok(result) => result,
                        Err(err) => {
                            println!("Websocket message handling error: {}", err);
                            let _ = sender.send(Message::Text(err));
                            return;
                        }
                    };
                    tx = Some(new_tx);
                }
            };
        } else {
            println!("Wrong format.");
            break;
        }
    }


    // websocket state handler
    let tx = tx.unwrap();
    let mut rx = tx.subscribe();

    let _ = tx.send("player joined the lobby".to_string());

    let mut recv_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
        // while let Ok(msg) = rx.recv().await {
        //     let request: WSGameTurnBody = match from_str(&msg) {
        //         Ok(req) => req,
        //         Err(err) => {
        //             println!("Connection request parsing error: {}", err);
        //             let _ = sender.send(Message::from("Failed to connect to room!")).await;
        //             break
        //         }
        //     };
        //     println!("{:#?}", request);
        //     message_body_handler(&request, &mut sender, app_state.clone()).await;
        // }
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

    // If any one of the tasks exit, abort the other.

    // returning from the handler closes the websocket connection
    if current_lobby_id.is_some() && client_game_id.is_some() {
        ws_leave_lobby(&app_state, &current_lobby_id.unwrap(), &client_game_id.unwrap()).await?
    } else {
        println!("Websocket connection closing error:");
    }

    println!("Websocket context {who} destroyed");
}
