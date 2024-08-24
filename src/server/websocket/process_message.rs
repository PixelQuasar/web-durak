use crate::lobby::PopulatedLobby;
use crate::server::controllers::lobby_controller::{
    get_lobby_by_id, get_populated_lobby, save_lobby,
};
use crate::server::controllers::player_controller::add_player_score;
use crate::server::errors::error_message;
use crate::server::websocket::client_request::{ClientRequest, ClientRequestType};
use crate::server::websocket::websocket_service::{ws_create_lobby, ws_join_lobby, ws_leave_lobby};
use crate::server::websocket::{
    GameEntityType, GameUpdateState, WSBody, WSBodyCardContent, WSGameUpdateResponseType,
    WSRequestType,
};
use crate::server::AppState;
use serde_json::{from_str, to_string};
use std::sync::Arc;
use tokio::sync::broadcast;

pub async fn handle_player_join(
    app_state: &Arc<AppState>,
    request: WSBody,
) -> Result<(PopulatedLobby, broadcast::Sender<String>), String> {
    let (lobby, sender) = match request.req_type {
        WSRequestType::LobbyJoin => ws_join_lobby(&app_state, request.clone()).await?,
        WSRequestType::LobbyCreate => ws_create_lobby(&app_state, request.clone()).await?,
        _ => return Err("Lobby connection error: wrong request type".to_string()),
    };

    let lobby = get_populated_lobby(&app_state.redis_pool, lobby.get_id()).await?;

    Ok((lobby, sender))
}

pub async fn handle_message(
    app_state: &Arc<AppState>,
    request: WSBody,
) -> Result<(ClientRequestType, String), String> {
    if request.lobby_id.is_none() {
        return Err("no lobby".to_string());
    }

    let lobby_id = request.clone().lobby_id.unwrap();

    let mut lobby = get_lobby_by_id(&app_state.redis_pool, &lobby_id.clone()).await?;

    println!("{:#?}", request.req_type);

    if request.req_type == WSRequestType::GameCreate {
        lobby.init_game(lobby.player_list().clone());

        save_lobby(&app_state.redis_pool, lobby.clone()).await?;

        let result = to_string::<PopulatedLobby>(
            &get_populated_lobby(&app_state.redis_pool, lobby_id.as_str())
                .await
                .map_err(|_| "game machine error")?,
        )
        .unwrap();

        Ok((ClientRequestType::GameCreate, result))
    } else if request.req_type == WSRequestType::GameFinish {
        lobby.finish_game();

        save_lobby(&app_state.redis_pool, lobby.clone()).await?;

        let result = to_string::<PopulatedLobby>(
            &get_populated_lobby(&app_state.redis_pool, lobby_id.as_str())
                .await
                .map_err(|_| "game machine error")?,
        )
        .unwrap();

        Ok((ClientRequestType::GameDelete, result))
    } else {
        match lobby.game {
            Some(ref mut game) => {
                if request.content.is_none() {
                    return Err("no content".to_string());
                }

                let game_content = from_str::<WSBodyCardContent>(&request.content.unwrap())
                    .map_err(error_message)?;

                let game_update_state = match request.req_type {
                    WSRequestType::GameTurnInitTable => {
                        if game_content.card.is_none() || game_content.player_id.is_none() {
                            return Err("no card".to_string());
                        }

                        let player_id = game_content.player_id.unwrap();

                        let card = game_content.card.unwrap();

                        game.init_table(&player_id, card.clone())
                            .map_err(|_| "game machine error")?;

                        vec![GameUpdateState::new(
                            GameEntityType::Table,
                            GameEntityType::Player,
                            "0".to_string(),
                            player_id,
                            vec![card],
                        )]
                    }
                    WSRequestType::GameTurnBeat => {
                        if game_content.beatable.is_none()
                            || game_content.beating.is_none()
                            || game_content.player_id.is_none()
                        {
                            return Err("invalid cards".to_string());
                        }

                        let player_id = game_content.player_id.unwrap();

                        let card = game_content.beating.unwrap();

                        let table_element_id = game
                            .beat(&player_id, card.clone(), game_content.beatable.unwrap())
                            .map_err(|_| "game machine error")?;

                        vec![GameUpdateState::new(
                            GameEntityType::Table,
                            GameEntityType::Player,
                            table_element_id.to_string(),
                            player_id,
                            vec![card],
                        )]
                    }
                    WSRequestType::GameTurnToss => {
                        if game_content.card.is_none() || game_content.player_id.is_none() {
                            return Err("no card".to_string());
                        }

                        let player_id = game_content.player_id.unwrap();

                        let card = game_content.card.unwrap();

                        let table_element_id = game
                            .toss(&player_id, card.clone())
                            .map_err(|_| "game machine error")?;

                        vec![GameUpdateState::new(
                            GameEntityType::Table,
                            GameEntityType::Player,
                            table_element_id.to_string(),
                            player_id,
                            vec![card],
                        )]
                    }
                    WSRequestType::GameTurnTransfer => {
                        if game_content.card.is_none() || game_content.player_id.is_none() {
                            return Err("invalid cards".to_string());
                        }

                        let player_id = game_content.player_id.unwrap();

                        let card = game_content.card.unwrap();

                        game.transfer(&player_id, card.clone())
                            .map_err(|_| "game machine error")?;

                        let table_element_id = game.table_size();

                        vec![GameUpdateState::new(
                            GameEntityType::Table,
                            GameEntityType::Player,
                            table_element_id.to_string(),
                            player_id,
                            vec![card],
                        )]
                    }
                    WSRequestType::GameTurnDiscard => {
                        let table_size = game.deck_manager.get_table_size();

                        let mut result = (0..table_size)
                            .map(|index| {
                                GameUpdateState::new(
                                    GameEntityType::Discard,
                                    GameEntityType::Table,
                                    String::new(),
                                    index.to_string(),
                                    game.deck_manager.get_table_element_cards(index),
                                )
                            })
                            .collect::<Vec<GameUpdateState>>();

                        game.finish_with_discard()
                            .map_err(|_| "game machine error")?;

                        let cards = game
                            .deal_more(&game.target_player_id().unwrap())
                            .map_err(|_| "game machine error")?;

                        for (player, hand) in cards {
                            result.push(GameUpdateState::new(
                                GameEntityType::Player,
                                GameEntityType::Deck,
                                player,
                                String::new(),
                                hand,
                            ));
                        }

                        result
                    }
                    WSRequestType::GameTurnTake => {
                        let player_id = game.target_player_id();

                        if player_id.is_none() {
                            return Err("no defender".to_string());
                        }

                        let player_id = player_id.unwrap();

                        let table_size = game.deck_manager.get_table_size();

                        let mut result = (0..table_size)
                            .map(|index| {
                                GameUpdateState::new(
                                    GameEntityType::Player,
                                    GameEntityType::Table,
                                    player_id.clone(),
                                    index.to_string(),
                                    game.deck_manager.get_table_element_cards(index),
                                )
                            })
                            .collect::<Vec<GameUpdateState>>();

                        game.finish_with_take().map_err(|_| "game machine error")?;

                        let cards = game
                            .deal_more(&game.target_player_id().unwrap())
                            .map_err(|_| "game machine error")?;

                        for (player, hand) in cards {
                            result.push(GameUpdateState::new(
                                GameEntityType::Player,
                                GameEntityType::Deck,
                                player,
                                String::new(),
                                hand,
                            ));
                        }

                        result
                    }
                    WSRequestType::GameTurnConfirmBeat => {
                        if game_content.player_id.is_none() {
                            return Err("no player".to_string());
                        }

                        game.confirm_beat(&game_content.player_id.unwrap())
                            .map_err(|_| "game machine error")?;

                        if game.is_all_confirmed() {
                            let table_size = game.deck_manager.get_table_size();

                            let mut result = (0..table_size)
                                .map(|index| {
                                    GameUpdateState::new(
                                        GameEntityType::Discard,
                                        GameEntityType::Table,
                                        String::new(),
                                        index.to_string(),
                                        game.deck_manager.get_table_element_cards(index),
                                    )
                                })
                                .collect::<Vec<GameUpdateState>>();

                            game.finish_with_discard()
                                .map_err(|_| "game machine error")?;

                            let cards = game
                                .deal_more(&game.target_player_id().unwrap())
                                .map_err(|_| "game machine error")?;

                            for (player, hand) in cards {
                                result.push(GameUpdateState::new(
                                    GameEntityType::Player,
                                    GameEntityType::Deck,
                                    player,
                                    String::new(),
                                    hand,
                                ));
                            }

                            result
                        } else {
                            vec![GameUpdateState::new(
                                GameEntityType::Nobody,
                                GameEntityType::Nobody,
                                String::new(),
                                String::new(),
                                vec![],
                            )]
                        }
                    }
                    _ => {
                        println!("Unknown state");
                        vec![GameUpdateState::new(
                            GameEntityType::Nobody,
                            GameEntityType::Nobody,
                            String::new(),
                            String::new(),
                            vec![],
                        )]
                    }
                };

                let can_be_finished = game.can_be_finished();

                if can_be_finished {
                    game.finish_game();

                    let populated_lobby =
                        get_populated_lobby(&app_state.redis_pool, &lobby_id).await?;

                    let scoreboard = game.get_leaderboard(populated_lobby.player_list());

                    for (mut player, score) in scoreboard {
                        add_player_score(&app_state.redis_pool, player.get_id().to_string(), score)
                            .await?;
                    }
                }

                save_lobby(&app_state.redis_pool, lobby.clone()).await?;

                let lobby = get_populated_lobby(&app_state.redis_pool, lobby_id.as_str())
                    .await
                    .map_err(|_| "game machine error")?;

                let response = WSGameUpdateResponseType::new(lobby, game_update_state);

                let result = to_string::<WSGameUpdateResponseType>(&response)
                    .map_err(|_| "parsing error")?;

                if !can_be_finished {
                    Ok((ClientRequestType::GameUpdate, result))
                } else {
                    Ok((ClientRequestType::GameFinish, result))
                }
            }

            None => return Err("error: no lobby".to_string()),
        }
    }
}

pub async fn disconnect_message(
    app_state: &Arc<AppState>,
    current_lobby_id: Option<String>,
    current_player_id: Option<String>,
) -> Result<String, String> {
    if current_lobby_id.is_some() && current_player_id.is_some() {
        let lobby = ws_leave_lobby(
            &app_state,
            &current_lobby_id.unwrap(),
            &current_player_id.unwrap(),
        )
        .await?;

        let lobby = get_populated_lobby(&app_state.redis_pool, lobby.get_id()).await?;

        Ok(ClientRequest::new(
            ClientRequestType::LobbyUpdate,
            to_string::<PopulatedLobby>(&lobby).unwrap(),
        )
        .to_string())
    } else {
        println!("Websocket connection closing error:");
        Err("Invalid player or lobby".to_string())
    }
}
