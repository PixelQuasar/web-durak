import {scoreBoardQuery} from "../../state/lobby-handler.js";
import {disconnectWebsocket} from "../../websocket/index.js";
import {navigate} from "../../utils/index.js";
import {getUser} from "../../state/index.js";
import {wsCreateGame, wsFinishGame} from "../../websocket/handle-game.js";

/**
 * Fetch scoreboard data and write it to html
 * @return {Promise<void>}
 */
const fetchScoreData = async function () {
    const data = await scoreBoardQuery(window.lobbyData.id);

    console.log(data);

    const container = document.querySelector(".score-container");

    container.innerHTML = data.map((item, index) =>
`<div class="score-item">
    <div class="place">${index + 1}</div>
    <div class="name">${item[0].name}</div>
    <div class="score">${item[1]}</div>
</div>`
    ).join("");
}

/**
 * Leave lobby and go to homepage
 */
const leaveLobbyAction = function () {
    disconnectWebsocket();
    navigate("/");
}

/**
 * Finish game and go to lobby page
 */
const finishGameAction = function () {
    wsFinishGame();
}

/**
 * Returns victory page.
 * @returns {string}
 * @constructor
 */
export const VictoryPage = function () {
    fetchScoreData();

    return `
<div>
    <h1>Game finished!</h1>
    <h2>Scores:</h2>
    <div class="score-container"></div>
    <button class="leave-button">LEAVE</button>
    ${window.lobbyData.owner_id === getUser() ? 
        `<button class="finish-game-button">RETURN TO LOBBY</button>` : 
        `<div>Waiting for host...</div>`
    }
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /leave-button/.test(className): leaveLobbyAction(); break;
        case /finish-game-button/.test(className): finishGameAction(); break;
    }
});
