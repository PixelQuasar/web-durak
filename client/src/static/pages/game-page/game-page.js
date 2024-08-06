import {templates} from "./table-templates.js"
import {navigate, PAGE_RENDER_EVENT_ID} from "../../utils/index.js";
import {disconnectWebsocket} from "../../websocket/index.js";

/**
 * A playing card.
 * @typedef {Object} Card
 * @property {number} r - rank
 * @property {number} s - suit
 */


/**
 * Playing hand data
 * @typedef {Object} HandData
 * @property {number} x
 * @property {number} y
 * @property {number} dir
 * @property {string} id
 * @property {string} playerId
 * @property {Card[]} cards
 */

const leaveLobbyAction = function () {
    disconnectWebsocket();
    navigate("/");
}


/**
 * Fetches hands data from game data
 * @param {Map<string, Card[]>} hands
 * @param {string[]} order
 * @param {string} playerId
 * @return {HandData[]}
 */
const fetchHandData = function (hands, order, playerId) {
    while (order[0] !== playerId) {
        order.push(order[0]);
        order.splice(0, 1);
    }

    const ww = window.innerWidth;
    const wh = window.innerHeight;
    const template = templates[order.length - 1];
    const templateLen = template.length;
    const handsData = new Array(order.length).fill(null);
    const gap = 128;

    for (let j = 0; j < templateLen; j++) {
        for (let i = 0; i < templateLen; i++) {
            if (template[i][j] !== '0') {
                const num = Number(template[i][j]);

                const dir = i === 0 ? 3 : i === templateLen - 1 ? 1 : j === 0 ? 2 : 4;

                const hGap = dir === 2 ? -gap : dir === 4 ? gap : 0;
                const vGap = dir === 1 ? -gap : dir === 3 ? gap : 0;

                handsData[num - 1] = {
                    x: j * ww / templateLen + hGap,
                    y: i * wh / templateLen + vGap,
                    dir: dir,
                    id: `hand-container-${num}`,
                    playerId: order[num - 1],
                    cards: hands[order[num - 1]]
                };
            }
        }
    }

    return handsData;
}


/**
 * Renderers playing hands by hands data.
 * @param {HandData[]} handsData
 * @return {string}
 */
const renderHands = function(handsData) {
    return `${handsData.map((item, index) => `
<div class="hand-container" id="${item.id}"></div>
`).join("")}`
}

/**
 * @returns {string}
 */
export const GamePage = function () {
    const lobbyData = window.lobbyData ?? {
        id: "UNKNOWN",
        status: "INACTIVE",
        public: false,
        player_list: [],
        game: null
    };

    const gameData = lobbyData.game;

    const handsData = fetchHandData(
        gameData.deck_manager.hands,
        gameData.deck_manager.hands_order,
        gameData.target_player_id
    );

    console.log(handsData);

    document.addEventListener(PAGE_RENDER_EVENT_ID, () => {
        console.log("event");

        for (const item of handsData) {
           const HTMLHand = document.querySelector(`#${item.id}`);

           console.log(item.id);

           HTMLHand.style.top = item.y + "px";

           HTMLHand.style.left = item.x + "px";

           HTMLHand.innerHTML = item.playerId;
        }
    })

    return `
<div class="game-page-wrapper page-wrapper">
    <div class="lobby-header">
        <div class="title">
            WEB DURAK
        </div>
        <button class="leave-button"> LEAVE </button>
    </div>
    
    <div class="lobby-title">
        LOBBY ${lobbyData.id} (public: ${lobbyData.public})
    </div>

    ${JSON.stringify(lobbyData.game)}
    ${renderHands(handsData)}
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /leave-button/.test(className): leaveLobbyAction(); break;
    }
});
