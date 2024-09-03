import {initWebsocketConnection} from "./init.js";
import {getUser} from "../state/index.js";
import {buildWebsocketRequest, ReqTypes} from "./request-builder.js";
import {navigate} from "../utils/index.js";

/**
 * Websocket endpoint to join existing lobby.
 * @param {string} lobbyId
 */
export const wsJoinLobby = function (lobbyId) {
    if (!window.websocket) return initWebsocketConnection();
    if (!getUser()) return navigate(`/create-user/${lobbyId}`);

    window.lobbyData = null;
    window.websocket.send(buildWebsocketRequest(ReqTypes.LobbyJoin, lobbyId, {}));
}

/**
 * Websocket endpoint to create new lobby.
 * @param {boolean} isPublic
 */
export const wsCreateLobby = function (isPublic) {
    if (!window.websocket) return initWebsocketConnection();
    if (!getUser()) return navigate("/create-user");

    window.lobbyData = null;
    window.websocket.send(buildWebsocketRequest(ReqTypes.LobbyCreate, "", { public: isPublic }));
}
