import {navigate} from "../../utils/index.js";
import {disconnectWebsocket} from "../../websocket/index.js";

const leaveLobby = function () {
    disconnectWebsocket();
    navigate("/");
}

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
<div class="lobby-page-wrapper page-wrapper">
    <div class="lobby-header">
        <div class="title">
            WEB DURAK
        </div>
        <button class="leave-button"> LEAVE </button>
    </div>
   
    <div class="subtitle">
        The layout is not final.
        Lobby id: ${lobbyData.id}.
        Players: ${lobbyData.player_list.join(" ")}.
    </div>
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /leave-button/.test(className): leaveLobby(); break;
        default: break;
    }
});
