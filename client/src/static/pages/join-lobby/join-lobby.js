import {initWebsocketConnection, wsCreateLobby, wsJoinLobby} from "../../websocket/index.js";

const joinLobbyAction = function () {
    let lobbyId = document.querySelector("#join-lobby-input").value;
    wsJoinLobby(lobbyId);
}

/**
 * Returns joining lobby page.
 * @returns {string}
 */
export const JoinLobby = function () {
    initWebsocketConnection();
    return `
<div class="join-lobby-wrapper page-wrapper">
   <div class="title">
        WEB DURAK
    </div>
    <div class="subtitle">
        The layout is not final.
    </div>
    
    <input type="text" id="join-lobby-input" class="large-textbox" placeholder="enter lobby code"/>
   <button class="join-button">JOIN LOBBY</button>
   <div class="error-msg"></div>
</div>
    `
}

document.querySelector("body").addEventListener("click", (event) => {
    switch (event.target.className) {
        case "join-button": joinLobbyAction(); break;
        default: break;
    }
})

document.querySelector("body").addEventListener("keypress", (event) => {
    if (event.key !== "enter") return;

    switch (event.target.className) {
        case "join-lobby-textbox": joinLobbyAction(); break;
    }
})
