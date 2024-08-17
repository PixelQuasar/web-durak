import {navigate, WEBSOCKET_UPDATE_ID} from "../utils/index.js";
import {CardType, moveCard, positionAllCards, updateGameData} from "../pages/game-page/game-page.js";

/**
 * server game update state entity
 * @typedef {Object} GameUpdateState
 * @property {string} receiver_type
 * @property {string} sender_type
 * @property {string} receiver_id
 * @property {string} sender_id
 * @property {Card[]} cards
 */

/**
 * Game update state handler. Mutates game-page gameData object.
 * @param {GameUpdateState} state
 */
const handleGameUpdateState = function (state) {
    const actionTypeMap = {
        "Player": CardType.Player,
        "Deck": CardType.Deck,
        "Discard": CardType.Discard,
        "Table": CardType.Table,
        "Nobody": CardType.Nobody
    }

    for (const card of state.cards) {
        moveCard(card, actionTypeMap[state.receiver_type], state.sender_id, state.receiver_id);
    }

    positionAllCards();
}

export const handleServerMessage = async function (data) {
    data = JSON.parse(data);

    // console.log("NEW WEBSOCKET DATA: ", data);

    if (!data.req_type && !data.content) return;

    switch (data.req_type) {
        case "LobbyUpdate": {
            let firstLobbyMsg = false;
            if (!window.lobbyData) firstLobbyMsg = true;
            window.lobbyData = JSON.parse(data.content);
            const websocketEvent = new Event(WEBSOCKET_UPDATE_ID);
            window.dispatchEvent(websocketEvent);
            if (firstLobbyMsg) navigate("/lobby");
            break;
        }
        case "GameCreate": {
            window.lobbyData = JSON.parse(data.content);
            const websocketEvent = new Event(WEBSOCKET_UPDATE_ID);
            window.dispatchEvent(websocketEvent);
            navigate("/game");
            break;
        }
        case "GameUpdate": {
            window.lobbyData = JSON.parse(data.content).lobby;
            updateGameData(window.lobbyData.game);
            const gameUpdateState = JSON.parse(data.content).game_update_states;
            for (const state of gameUpdateState) {
                handleGameUpdateState(state);
            }
            break;
        }
        case "GameFinish": {
            break;
        }
    }
}
