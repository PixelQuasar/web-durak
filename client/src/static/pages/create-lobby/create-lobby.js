import {initWebsocketConnection, wsCreateLobby} from "../../websocket/index.js";

const createLobbyAction = function () {
    wsCreateLobby(true);
}

export const CreateLobby = function () {
    initWebsocketConnection();
    return `
<div class="create-lobby-wrapper">
   <div class="title">
        WEB DURAK
    </div>
    <div class="subtitle">
        The layout is not final.
    </div>
    
    
   <button class="create-lobby-button">CREATE LOBBY</button>
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    switch (event.target.className) {
        case "create-lobby-button": createLobbyAction(); break;
        default: break;
    }
});
