import {initWebsocketConnection, wsCreateLobby} from "../../websocket/index.js";

const createLobbyAction = function () {
    wsCreateLobby(true);
}

/**
 * Returns creating lobby page.
 * @returns {string}
 */
export const CreateLobby = function () {
    initWebsocketConnection();
    return `
<div class="create-lobby-wrapper page-wrapper">
   <h1 class="page-title">WEB DUR<span>A</span>K</h1>
    <div class="subtitle">
        The layout is not final.
    </div>
    
    
   <button class="create-lobby-button">CREATE LOBBY</button>
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    switch (event.target.className) {
        case "create-lobby-button": createLobbyAction(); break;
    }
});
