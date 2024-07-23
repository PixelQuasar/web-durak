import {navigate} from "../../utils/index.js";
import {disconnectWebsocket} from "../../websocket/index.js";

const leaveLobby = function () {
    disconnectWebsocket();
    navigate("/");
}

export const LobbyPage = function () {
    const lobbyData = window.lobbyData ?? {
            id: "UNKNOWN",
            status: "INACTIVE",
            public: false,
            player_list: [],
            game: null
    };

    return `
<div class="lobby-page-wrapper page-wrapper">
    <div class="lobby-header">
        <div class="title">
            WEB DURAK
        </div>
        <button class="leave-button"> LEAVE </button>
    </div>
    
    <div class="lobby-title">
        LOBBY ${lobbyData.id} (public: ${lobbyData.public})
    </div>
    
    
    <div class="lobby-wrapper">
        <div class="player-list">
            <div class="title">
                Players:
            </div>
            ${lobbyData.player_list.map(item => `
                <div class="player-list-item"> ${item.name} </div>
            `).join("")}
        </div>
        
        <div class="game-info">
            <button class="create-game-button"> CREATE GAME </button>
        </div>
    </div>
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /leave-button/.test(className): leaveLobby(); break;
    }
});
