import {initWebsocketConnection} from "./init.js";
import {getUser} from "../state/index.js";
import {goToSignup} from "../utils/index.js";
import {buildWebsocketRequest, ReqTypes} from "./request-builder.js";

export const wsJoinLobby = function (lobbyId) {
    if (!window.websocket) return initWebsocketConnection();
    if (!getUser()) return goToSignup();

    window.lobbyData = null;
    window.websocket.send(buildWebsocketRequest(ReqTypes.JoinLobby, lobbyId, {}));
}

export const wsCreateLobby = function (isPublic) {
    if (!window.websocket) return initWebsocketConnection();
    if (!getUser()) return goToSignup();

    window.lobbyData = null;
    window.websocket.send(buildWebsocketRequest(ReqTypes.CreateLobby, "", { public: isPublic }));
}
