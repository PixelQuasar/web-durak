import {getUser} from "../state/index.js";

export const ReqTypes = {
    LobbyCreate: "LobbyCreate",
    LobbyJoin: "LobbyJoin",
    GameCreate: "GameCreate",
    GameTurnInitTable: "GameTurnInitTable",
    GameTurnToss: "GameTurnToss",
    GameTurnBeat: "GameTurnBeat",
    GameTurnTake: "GameTurnTake",
    GameTurnDiscard: "GameTurnDiscard",
    GameTurnConfirmBeat: "GameTurnConfirmBeat"
}

/**
 * Builds JSON body that would be correctly read by server--side websocket listener.
 * @param {string} type
 * @param {string} lobbyId
 * @param {any} content
 * @returns {string}
 */
export const buildWebsocketRequest = function (type, lobbyId, content) {
    return JSON.stringify({
        req_type: type,
        sender_id: getUser(),
        lobby_id: lobbyId,
        content: JSON.stringify(content)
    })
}
