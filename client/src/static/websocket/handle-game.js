import {initWebsocketConnection} from "./init.js";
import {buildWebsocketRequest, ReqTypes} from "./request-builder.js";
import {navigate} from "../utils/index.js";

/**
 * Create game websocket endpoint.
 */
export const wsCreateGame = function () {
    if (!window.websocket) return initWebsocketConnection();
    if (!window.lobbyData) return navigate("/");
    console.log(window.lobbyData);
    window.websocket.send(buildWebsocketRequest(ReqTypes.GameCreate, window.lobbyData.id, {}));
}
