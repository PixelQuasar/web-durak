import {initWebsocketConnection} from "../../websocket/index.js";

export const LobbyPage = function () {
    console.log(window.lobbyData);
    const lobbyData = window.lobbyData ?? {
            id: "UNKNOWN",
            status: "INACTIVE",
            public: false,
            player_list: [],
            game: null
    };

    console.log(lobbyData["id"]);
    console.log(lobbyData.player_list);

    return `
<div class="lobby-page-wrapper">
   <div class="title">
        WEB DURAK
    </div>
    <div class="subtitle">
        The layout is not final.
        Lobby id: ${lobbyData.id}.
        Players: ${lobbyData.player_list.join(" ")}.
    </div>
</div>`
}
