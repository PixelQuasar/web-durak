import {getUser} from "../state/index.js";

export const ReqTypes = {
    JoinLobby: "JoinLobby",
    CreateLobby: "CreateLobby"
}

export const buildWebsocketRequest = function (type, lobbyId, content) {
    return JSON.stringify({
        req_type: type,
        sender_id: getUser(),
        lobby_id: lobbyId,
        content: JSON.stringify(content)
    })
}
