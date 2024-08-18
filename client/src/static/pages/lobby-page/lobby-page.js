import {navigate} from "../../utils/index.js";
import {disconnectWebsocket} from "../../websocket/index.js";
import {wsCreateGame} from "../../websocket/handle-game.js";
import {getLobby} from "../../state/lobby-handler.js";
import {getUser} from "../../state/index.js";

/**
 * Leave lobby and go to homepage
 */
const leaveLobbyAction = function () {
    disconnectWebsocket();
    navigate("/");
}

/**
 * Create game
 */
const createGameAction = function () {
    wsCreateGame();
}

/**
 * Returns page that indicates lobby info.
 * @returns {string}
 */
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
                Players (${window.lobbyData.player_list.length}):
            </div>
            ${lobbyData.player_list.map(item => `
                <div class="player-list-item"> ${item.name} </div>
            `).join("")}
        </div>
        
        <div class="game-info">
            ${window.lobbyData.owner_id === getUser() ? 
                `<button class="create-game-button"> CREATE GAME </button>` : 
                `<h2>Waiting for host to start a game...</h2>`
            }
        </div>
    </div>
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /leave-button/.test(className): leaveLobbyAction(); break;
        case /create-game-button/.test(className): createGameAction(); break;
    }
});
