import {initWebsocketConnection} from "./init.js";
import {buildWebsocketRequest, ReqTypes} from "./request-builder.js";
import {navigate} from "../utils/index.js";

/**
 * Create game websocket endpoint.
 */
export const wsCreateGame = function () {
    if (!window.websocket) return initWebsocketConnection();
    if (!window.lobbyData) return navigate("/");
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameCreate, window.lobbyData.id, {}));
}

export const wsFinishGame = function () {
    if (!window.websocket) return initWebsocketConnection();
    if (!window.lobbyData) return navigate("/");
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameFinish, window.lobbyData.id, {}));
}

export const wsGameConfirmPass = function (playerId) {
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameTurnConfirmBeat, window.lobbyData.id, {
        player_id: playerId
    }));
}

export const wsGameInitTurn = function (card, playerId) {
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameTurnInitTable, window.lobbyData.id, {
        card, player_id: playerId
    }));
}

export const wsGameBeat = function (beating, beatable, playerId) {
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameTurnBeat, window.lobbyData.id, {
        beating, beatable, player_id: playerId
    }));
}

export const wsGameToss = function (card, playerId) {
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameTurnToss, window.lobbyData.id, {
        card, player_id: playerId
    }));
}

export const wsGameTransfer = function (card, playerId) {
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameTurnTransfer, window.lobbyData.id, {
        card, player_id: playerId
    }));
}

export const wsGameDiscard = function () {
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameTurnToss, window.lobbyData.id, {}));
}

export const wsGameTake = function (playerId) {
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameTurnTake, window.lobbyData.id, {
        player_id: playerId
    }));
}
