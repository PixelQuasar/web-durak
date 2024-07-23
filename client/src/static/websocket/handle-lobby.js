import {initWebsocketConnection} from "./init.js";
import {getUser} from "../state/index.js";
import {buildWebsocketRequest, ReqTypes} from "./request-builder.js";
import {navigate} from "../utils/index.js";

export const wsJoinLobby = function (lobbyId) {
    if (!window.websocket) return initWebsocketConnection();
    if (!getUser()) return navigate("/create-user");

    window.lobbyData = null;
    window.websocket.send(buildWebsocketRequest(ReqTypes.JoinLobby, lobbyId, {}));
}

export const wsCreateLobby = function (isPublic) {
    if (!window.websocket) return initWebsocketConnection();
    if (!getUser()) return navigate("/create-user");

    window.lobbyData = null;
    window.websocket.send(buildWebsocketRequest(ReqTypes.CreateLobby, "", { public: isPublic }));
}
