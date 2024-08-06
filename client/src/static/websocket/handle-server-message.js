import {navigate, WEBSOCKET_UPDATE_ID} from "../utils/index.js";
import {getLobbyQuery} from "../state/lobby-handler.js";

export const handleServerMessage = async function (data) {
    data = JSON.parse(data);

    // console.log(data);

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
    }
}
