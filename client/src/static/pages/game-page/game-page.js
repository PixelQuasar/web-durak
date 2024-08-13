import {templates} from "./table-templates.js"
import {navigate, PAGE_RENDER_EVENT_ID, triggerRenderPageId} from "../../utils/index.js";
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

/**
 * Playing table data
 * @typedef {Object} TableData
 * @property {Card[]} deck
 * @property {number} trump_suit
 * @property {Card[]} discard
 * @property {[Card, Card][]} table
 */

const leaveLobbyAction = function () {
    disconnectWebsocket();
    navigate("/");
}


/**
 * Fetches hands data from game data at page mount.
 * @param {Map<string, Card[]>} hands
 * @param {string[]} order
 * @param {string} playerId
 * @return {HandData[]}
 */
const fetchHandDataOnMount = function (hands, order, playerId) {
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

    const headerSize = 120;

    for (let j = 0; j < templateLen; j++) {
        for (let i = 0; i < templateLen; i++) {
            if (template[i][j] !== '0') {
                const num = Number(template[i][j]);

                const dir = i === 0 ? 3 : i === templateLen - 1 ? 1 : j === 0 ? 2 : 4;

                const hGap = dir === 4 ? -gap : dir === 2 ? gap : 0;
                const vGap = dir === 1 ? -gap : dir === 3 ? gap : 0;

                const offset = 75;

                const hOffset = dir === 1 ? offset : dir === 3 ? -offset : 0;
                const vOffset = dir === 2 ? offset : dir === 4 ? -offset : 0;


                handsData[num - 1] = {
                    x: j * ww / (templateLen - 1) + hGap / 10 - hOffset,
                    y: i * wh / (templateLen - 1) + vGap - vOffset - headerSize,
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
 * Draws and manages player hands styles
 * @param {HandData[]} handsData
 */
const postRenderHands = function (handsData) {
    for (const item of handsData) {
        const HTMLHand = document.querySelector(`#${item.id}`);

        HTMLHand.style.top = `${item.y}px`;

        HTMLHand.style.left = `${item.x}px`;

        HTMLHand.style.rotate = `${[0, 90, 180, 270][item.dir - 1]}deg`;

        let cardsHTML = "";

        for (let i = 0; i < item.cards.length; i++) {
            const card = item.cards[i];

            cardsHTML += `
<div class="card-container" id="card-container-${item.playerId}-${i}">
    ${card.r} ${card.s}
</div>
`
        }
        HTMLHand.innerHTML = cardsHTML;
    }

    triggerRenderPageId();
}

/**
 * Draws and manages playing cards styles
 * @param {HandData[]} handsData
 */
const postRenderCards = function (handsData) {
    for (const item of handsData) {
        const cardsHTMLIds = item.cards.map((_, i) =>  `card-container-${item.playerId}-${i}`);

        for (let i = 0; i < item.cards.length; i++) {
            const card = item.cards[i];

            const HTMLCard = document.querySelector(`#${cardsHTMLIds[i]}`);

            let cardGap = 40;

            if (item.id === "hand-container-1") {
                HTMLCard.classList.add("player-card-container");
                cardGap = 80;
            }

            HTMLCard.style.top = `${Math.abs((item.cards.length - 1) / 2 - i) * 10 - 50}px`;
            HTMLCard.style.left = `${((item.cards.length - 1) / 2 - i) * cardGap}px`;
            HTMLCard.style.rotate = `${(((item.cards.length - 1) / 2) - i) * 5}deg`;

            HTMLCard.addEventListener("mouseenter", () => {
                HTMLCard.style.transform = `translate(0, -100px) rotate(${(((item.cards.length - 1) / 2) - i) * -5}deg)`;
                const leftNeighbors = cardsHTMLIds.slice(0, i).map(x => document.getElementById(x));;
                const rightNeighbors = cardsHTMLIds.slice(i + 1).map(x => document.getElementById(x));;
                for (const neighbor of leftNeighbors) {
                    neighbor.style.transform = "translateX(10px)";
                }
                for (const neighbor of rightNeighbors) {
                    neighbor.style.transform = "translateX(-60px)";
                }
            })

            HTMLCard.addEventListener("mouseleave", () => {
                HTMLCard.style.transform = `translate(0, 0) rotate(0)`;
                const leftNeighbors = cardsHTMLIds.slice(0, i).map(x => document.getElementById(x));;
                const rightNeighbors = cardsHTMLIds.slice(i + 1).map(x => document.getElementById(x));
                for (const neighbor of leftNeighbors) {
                    neighbor.style.transform = "translateX(0)";
                }
                for (const neighbor of rightNeighbors) {
                    neighbor.style.transform = "translateX(0)";
                }
            })
        }
    }
}

/**
 * Renders playing table by table data
 * @param {TableData} tableData
 */
const renderTable = function (tableData) {

}


/**
 * returns playing hands elements by hands data.
 * @param {HandData[]} handsData
 * @return {string}
 */
const handsElements = function(handsData) {
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

    //const gameData = lobbyData.game;

    //console.log(JSON.stringify(gameData));

    const gameData = JSON.parse("{\"id\":\"GAME977547931\",\"status\":\"Start\",\"participant_ids\":[\"22233530764\",\"2404048658\",\"2734066259\",\"22483399774\"],\"attacker_player_id\":\"2734066259\",\"target_player_id\":\"2404048658\",\"next_player_id\":\"22233530764\",\"turn_queue\":[],\"deck_manager\":{\"full_deck\":[{\"s\":1,\"r\":1},{\"s\":2,\"r\":1},{\"s\":3,\"r\":1},{\"s\":4,\"r\":1},{\"s\":1,\"r\":2},{\"s\":2,\"r\":2},{\"s\":3,\"r\":2},{\"s\":4,\"r\":2},{\"s\":1,\"r\":3},{\"s\":2,\"r\":3},{\"s\":3,\"r\":3},{\"s\":4,\"r\":3},{\"s\":1,\"r\":4},{\"s\":2,\"r\":4},{\"s\":3,\"r\":4},{\"s\":4,\"r\":4},{\"s\":1,\"r\":5},{\"s\":2,\"r\":5},{\"s\":3,\"r\":5},{\"s\":4,\"r\":5},{\"s\":1,\"r\":6},{\"s\":2,\"r\":6},{\"s\":3,\"r\":6},{\"s\":4,\"r\":6},{\"s\":1,\"r\":7},{\"s\":2,\"r\":7},{\"s\":3,\"r\":7},{\"s\":4,\"r\":7},{\"s\":1,\"r\":8},{\"s\":2,\"r\":8},{\"s\":3,\"r\":8},{\"s\":4,\"r\":8},{\"s\":1,\"r\":9},{\"s\":2,\"r\":9},{\"s\":3,\"r\":9},{\"s\":4,\"r\":9},{\"s\":1,\"r\":10},{\"s\":2,\"r\":10},{\"s\":3,\"r\":10},{\"s\":4,\"r\":10},{\"s\":1,\"r\":11},{\"s\":2,\"r\":11},{\"s\":3,\"r\":11},{\"s\":4,\"r\":11},{\"s\":1,\"r\":12},{\"s\":2,\"r\":12},{\"s\":3,\"r\":12},{\"s\":4,\"r\":12},{\"s\":1,\"r\":13},{\"s\":2,\"r\":13},{\"s\":3,\"r\":13},{\"s\":4,\"r\":13}],\"deck\":[{\"s\":1,\"r\":2},{\"s\":2,\"r\":6},{\"s\":3,\"r\":12},{\"s\":2,\"r\":2},{\"s\":2,\"r\":10},{\"s\":4,\"r\":12},{\"s\":1,\"r\":1},{\"s\":4,\"r\":6},{\"s\":2,\"r\":3},{\"s\":3,\"r\":10},{\"s\":4,\"r\":13},{\"s\":3,\"r\":13},{\"s\":4,\"r\":3},{\"s\":2,\"r\":13},{\"s\":3,\"r\":6},{\"s\":4,\"r\":1},{\"s\":2,\"r\":7},{\"s\":2,\"r\":4},{\"s\":2,\"r\":11},{\"s\":1,\"r\":8},{\"s\":4,\"r\":9},{\"s\":2,\"r\":1},{\"s\":1,\"r\":5},{\"s\":4,\"r\":8},{\"s\":3,\"r\":9},{\"s\":3,\"r\":7},{\"s\":2,\"r\":12},{\"s\":3,\"r\":3}],\"discard\":[],\"hands\":{\"2404048658\":[{\"s\":2,\"r\":8},{\"s\":1,\"r\":12},{\"s\":3,\"r\":1},{\"s\":1,\"r\":11},{\"s\":2,\"r\":5},{\"s\":1,\"r\":10}],\"2734066259\":[{\"s\":2,\"r\":9},{\"s\":1,\"r\":7},{\"s\":4,\"r\":7},{\"s\":4,\"r\":10},{\"s\":1,\"r\":4},{\"s\":1,\"r\":13}],\"22233530764\":[{\"s\":3,\"r\":2},{\"s\":1,\"r\":3},{\"s\":4,\"r\":4},{\"s\":3,\"r\":8},{\"s\":3,\"r\":5},{\"s\":1,\"r\":9}],\"22483399774\":[{\"s\":1,\"r\":6},{\"s\":4,\"r\":11},{\"s\":3,\"r\":4},{\"s\":4,\"r\":5},{\"s\":3,\"r\":11},{\"s\":4,\"r\":2}]},\"hands_amount\":4,\"hands_order\":[\"2734066259\",\"2404048658\",\"22233530764\",\"22483399774\"],\"beat_confirmations\":{\"2404048658\":false,\"2734066259\":false,\"22233530764\":false,\"22483399774\":false},\"hand_size\":6,\"trump_suit\":3,\"table\":[]}}")

    const handsData = fetchHandDataOnMount(
        gameData.deck_manager.hands,
        gameData.deck_manager.hands_order,
        gameData.target_player_id
    );

    document.addEventListener(PAGE_RENDER_EVENT_ID, () => {
        if (document.querySelector(".hand-container")) {
            postRenderHands(handsData);
        }

        if (document.querySelector(".table-container")) {
            renderTable({
                table: gameData.deck_manager.table,
                trump_suit: gameData.deck_manager.trump_suit,
                deck: gameData.deck_manager.deck,
                discard: gameData.deck_manager.discard
            })
        }

        if (document.querySelector(".card-container")) {
            postRenderCards(handsData);
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
    <div class="game-container">
        ${renderHands(handsData)}
        <div class="table-container">
            <div class="deck-container"></div>
            <div class="table-container"></div>
            <div class="discard-container"></div>
        </div>
    </div>
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /leave-button/.test(className): leaveLobbyAction(); break;
    }
});
