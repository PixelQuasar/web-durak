import {initWebsocketConnection, wsJoinLobby} from "../../websocket/index.js";
import {getLobbiesListQuery} from "../../state/lobby-handler.js";
import {navigate, PAGE_RENDER_EVENT_ID} from "../../utils/index.js";
import {getUser} from "../../state/index.js";

/**
 * Join lobby
 */
const joinLobbyAction = function () {
    let lobbyId = document.querySelector("#join-lobby-input").value;
    wsJoinLobby(lobbyId);
}

const renderLobbies = async function () {
    const lobbies = await getLobbiesListQuery();

    let container = document.querySelector(".lobby-list");

    container.innerHTML = "";

    lobbies.filter((lobby) => lobby.status === "ACTIVE").forEach((lobby) => {
        container.innerHTML +=
`<div class="item">
    <div class="header">${lobby.owner_id.name}</div>
    <div class="players">${lobby.player_list.length}/6</div>
    <div class="button-container"><button lobby-id="${lobby.id}" class="button">JOIN</button></div>
</div>`
    });

    container.addEventListener("click", (event) => {
        if (event.target.classList.contains("button")) {
            wsJoinLobby(event.target.getAttribute("lobby-id"));
        }
    });
}

const handleParams = function (params) {
    if (params.lobbyid) {
        wsJoinLobby(params.lobbyid);
    }
}

/**
 * Returns joining lobby page.
 * @returns {string}
 */
export const JoinLobby = function () {
    initWebsocketConnection(() => {
        handleParams(this.params);
    });

    return `
<div class="join-lobby-wrapper page-wrapper">
    <h1 class="page-title">WEB DUR<span>A</span>K</h1>
    <div class="subtitle">The layout is not final.</div>
    <input type="text" id="join-lobby-input" class="large-textbox" placeholder="enter lobby code"/>
   <button class="join-button">JOIN LOBBY</button>
   <div class="error-msg"></div>
   <button class="button" id="reload-button">RELOAD</button>
   <div class="lobby-list"></div>
</div>`
}

document.addEventListener(PAGE_RENDER_EVENT_ID, () => {
    if (document.querySelector(".lobby-list")) {
        renderLobbies();
    }
    if (document.querySelector("#reload-button")) {
        document.querySelector("#reload-button").addEventListener("click", () => {
            renderLobbies();
        })
    }
})

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
