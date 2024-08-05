use std::sync::Arc;
use serde_json::{from_str, to_string};
use tokio::sync::broadcast;
use crate::lobby::{PopulatedLobby};
use crate::server::AppState;
use crate::server::controllers::lobby_controller::{get_lobby_by_id, get_populated_lobby, save_lobby};
use crate::server::errors::error_message;
use crate::server::websocket::{WSBody, WSBodyCardContent, WSRequestType};
use crate::server::websocket::client_request::{ClientRequest, ClientRequestType};
use crate::server::websocket::websocket_service::{ws_create_lobby, ws_join_lobby, ws_leave_lobby};

pub async fn handle_player_join(
    app_state: &Arc<AppState>,
    request: WSBody
) -> Result<(PopulatedLobby, broadcast::Sender<String>), String> {
    let (lobby, sender) = match request.req_type {
        WSRequestType::LobbyJoin => {
            ws_join_lobby(&app_state, request.clone()).await?
        },
        WSRequestType::LobbyCreate => {
            ws_create_lobby(&app_state, request.clone()).await?
        },
        _ => {
            return Err("Lobby connection error: wrong request type".to_string())
        }
    };

    let lobby = get_populated_lobby(&app_state.redis_pool, lobby.get_id()).await?;

    Ok((lobby, sender))
}

pub async fn handle_message(
    app_state: &Arc<AppState>,
    request: WSBody
) -> Result<String, String> {
    if request.lobby_id.is_none() {
        return Err("no lobby".to_string());
    }

    let lobby_id = request.lobby_id.unwrap();

    let mut lobby = get_lobby_by_id(&app_state.redis_pool, &lobby_id.clone()).await?;

    print!("{:#?}", lobby);

    if request.req_type == WSRequestType::GameCreate {
        lobby.init_game(lobby.player_list().clone());

        save_lobby(&app_state.redis_pool, lobby.clone()).await?;

        let result = to_string::<PopulatedLobby>(
            &get_populated_lobby(&app_state.redis_pool, lobby_id.as_str()).await
                .map_err(|_| { "game machine error" })?
        ).unwrap();

        Ok(result)
    }
    else {
        match lobby.game {
            Some(ref mut game) => {
                if request.content.is_none() {
                    return Err("no content".to_string());
                }

                let game_content = from_str::<WSBodyCardContent>(
                    &request.content.unwrap()).map_err(error_message
                )?;

                match request.req_type {
                    WSRequestType::GameTurnInitTable => {
                        if game_content.card.is_none() || game_content.player_id.is_none() {
                            return Err("no card".to_string());
                        }

                        game.init_table(&game_content.player_id.unwrap(), game_content.card.unwrap())
                            .map_err(|_| { "game machine error" })?;
                    },
                    WSRequestType::GameTurnBeat => {
                        if game_content.beatable.is_none() || game_content.beating.is_none() || game_content.player_id.is_none() {
                            return Err("invalid cards".to_string());
                        }

                        game.beat(
                            &game_content.player_id.unwrap(),
                            game_content.beating.unwrap(),
                            game_content.beatable.unwrap()
                        ).map_err(|_| { "game machine error" })?;
                    },
                    WSRequestType::GameTurnToss => {
                        if game_content.card.is_none() || game_content.player_id.is_none() {
                            return Err("no card".to_string());
                        }

                        game.toss(&game_content.player_id.unwrap(), game_content.card.unwrap())
                            .map_err(|_| { "game machine error" })?;
                    },
                    WSRequestType::GameTurnDiscard => {
                        game.toss(&game_content.player_id.unwrap(), game_content.card.unwrap())
                            .map_err(|_| { "game machine error" })?;
                    },
                    WSRequestType::GameTurnTake => {
                        if game.target_player_id().is_none() {
                            return Err("no defender".to_string());
                        }

                        game.toss(&game.target_player_id().unwrap(), game_content.card.unwrap())
                            .map_err(|_| { "game machine error" })?;
                    },
                    WSRequestType::GameTurnConfirmBeat => {
                        if game_content.player_id.is_none() {
                            return Err("no player".to_string());
                        }

                        game.confirm_beat(&game_content.player_id.unwrap())
                            .map_err(|_| { "game machine error" })?;
                    }
                    _ => {
                        println!("Unknown state");
                    }
                };
                save_lobby(&app_state.redis_pool, lobby.clone()).await?;

                let result = to_string::<PopulatedLobby>(
                    &get_populated_lobby(&app_state.redis_pool, lobby_id.as_str()).await
                        .map_err(|_| { "game machine error" })?
                ).unwrap();

                Ok(result)
            },
            None => {
                return Err("lobby error".to_string())
            }
        }
    }
}

pub async fn disconnect_message(
    app_state: &Arc<AppState>,
    current_lobby_id: Option<String>,
    current_player_id: Option<String>
) -> Result<String, String> {
    if current_lobby_id.is_some() && current_player_id.is_some() {
        let lobby = ws_leave_lobby(&app_state, &current_lobby_id.unwrap(), &current_player_id.unwrap()).await?;

        let lobby = get_populated_lobby(&app_state.redis_pool, lobby.get_id()).await?;

        Ok(ClientRequest::new(
            ClientRequestType::LobbyUpdate, to_string::<PopulatedLobby>(&lobby).unwrap()
        ).to_string())
    } else {
        println!("Websocket connection closing error:");
        Err("Websocket connection closing error: invalid player or lobby".to_string())
    }
}
