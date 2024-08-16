import {templates} from "./table-templates.js"
import {navigate, PAGE_RENDER_EVENT_ID, randomRange, triggerRenderPageId} from "../../utils/index.js";
import {disconnectWebsocket} from "../../websocket/index.js";
import {getUser} from "../../state/index.js";


/**
 * A playing card.
 * @typedef {Object} Card
 * @property {number} r - rank
 * @property {number} s - suit
 * @property {number} id - card id
 */

/**
 * Deck manager type
 * @typedef {Object} DeckManager
 * @property {Card[]} full_deck
 * @property {Card[]} deck
 * @property {Card[]} discard
 * @property {Map<string, Card[]>} hands
 * @property {number} hands_amount
 * @property {string[]} hands_order
 * @property {Map<string, boolean>} beat_confirmations
 * @property {number} hand_size
 * @property {number} trump_suit
 * @property {[Card, Card | null][]} table
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
 * Card render data
 * @typedef {Object} CardRenderData
 * @property {number} x
 * @property {number} y
 * @property {number} rotation
 * @property {boolean} facing
 */

/**
 * Playing table data
 * @typedef {Object} TableData
 * @property {Card[]} deck
 * @property {number} trumpSuit
 * @property {Card[]} discard
 * @property {[Card, Card][]} table
 */

/**
 * @type {Map<number, CardRenderData>}
 */
let cardRenderDataMap = new Map();

let gameData = JSON.parse("{\"id\":\"GAME4273841987\",\"status\":\"Start\",\"participant_ids\":[\"22233530764\",\"2404048658\",\"2734066259\",\"22483399774\"],\"attacker_player_id\":\"2734066259\",\"target_player_id\":\"2404048658\",\"next_player_id\":\"22233530764\",\"turn_queue\":[],\"deck_manager\":{\"full_deck\":[{\"s\":1,\"r\":1,\"id\":1},{\"s\":2,\"r\":1,\"id\":2},{\"s\":3,\"r\":1,\"id\":3},{\"s\":4,\"r\":1,\"id\":4},{\"s\":1,\"r\":2,\"id\":5},{\"s\":2,\"r\":2,\"id\":6},{\"s\":3,\"r\":2,\"id\":7},{\"s\":4,\"r\":2,\"id\":8},{\"s\":1,\"r\":3,\"id\":9},{\"s\":2,\"r\":3,\"id\":10},{\"s\":3,\"r\":3,\"id\":11},{\"s\":4,\"r\":3,\"id\":12},{\"s\":1,\"r\":4,\"id\":13},{\"s\":2,\"r\":4,\"id\":14},{\"s\":3,\"r\":4,\"id\":15},{\"s\":4,\"r\":4,\"id\":16},{\"s\":1,\"r\":5,\"id\":17},{\"s\":2,\"r\":5,\"id\":18},{\"s\":3,\"r\":5,\"id\":19},{\"s\":4,\"r\":5,\"id\":20},{\"s\":1,\"r\":6,\"id\":21},{\"s\":2,\"r\":6,\"id\":22},{\"s\":3,\"r\":6,\"id\":23},{\"s\":4,\"r\":6,\"id\":24},{\"s\":1,\"r\":7,\"id\":25},{\"s\":2,\"r\":7,\"id\":26},{\"s\":3,\"r\":7,\"id\":27},{\"s\":4,\"r\":7,\"id\":28},{\"s\":1,\"r\":8,\"id\":29},{\"s\":2,\"r\":8,\"id\":30},{\"s\":3,\"r\":8,\"id\":31},{\"s\":4,\"r\":8,\"id\":32},{\"s\":1,\"r\":9,\"id\":33},{\"s\":2,\"r\":9,\"id\":34},{\"s\":3,\"r\":9,\"id\":35},{\"s\":4,\"r\":9,\"id\":36},{\"s\":1,\"r\":10,\"id\":37},{\"s\":2,\"r\":10,\"id\":38},{\"s\":3,\"r\":10,\"id\":39},{\"s\":4,\"r\":10,\"id\":40},{\"s\":1,\"r\":11,\"id\":41},{\"s\":2,\"r\":11,\"id\":42},{\"s\":3,\"r\":11,\"id\":43},{\"s\":4,\"r\":11,\"id\":44},{\"s\":1,\"r\":12,\"id\":45},{\"s\":2,\"r\":12,\"id\":46},{\"s\":3,\"r\":12,\"id\":47},{\"s\":4,\"r\":12,\"id\":48},{\"s\":1,\"r\":13,\"id\":49},{\"s\":2,\"r\":13,\"id\":50},{\"s\":3,\"r\":13,\"id\":51},{\"s\":4,\"r\":13,\"id\":52}],\"deck\":[{\"s\":1,\"r\":1,\"id\":1},{\"s\":4,\"r\":6,\"id\":24},{\"s\":2,\"r\":3,\"id\":10},{\"s\":3,\"r\":10,\"id\":39},{\"s\":4,\"r\":13,\"id\":52},{\"s\":3,\"r\":13,\"id\":51},{\"s\":4,\"r\":3,\"id\":12},{\"s\":2,\"r\":13,\"id\":50},{\"s\":3,\"r\":6,\"id\":23},{\"s\":4,\"r\":1,\"id\":4},{\"s\":2,\"r\":7,\"id\":26},{\"s\":2,\"r\":4,\"id\":14},{\"s\":2,\"r\":11,\"id\":42},{\"s\":1,\"r\":8,\"id\":29},{\"s\":4,\"r\":9,\"id\":36},{\"s\":2,\"r\":1,\"id\":2},{\"s\":1,\"r\":5,\"id\":17},{\"s\":4,\"r\":8,\"id\":32},{\"s\":3,\"r\":9,\"id\":35}],\"discard\":[{\"s\":3,\"r\":7,\"id\":27},{\"s\":2,\"r\":12,\"id\":46},{\"s\":3,\"r\":3,\"id\":11}],\"hands\":{\"2404048658\":[{\"s\":2,\"r\":8,\"id\":30},{\"s\":1,\"r\":12,\"id\":45},{\"s\":3,\"r\":1,\"id\":3},{\"s\":1,\"r\":11,\"id\":41},{\"s\":2,\"r\":5,\"id\":18},{\"s\":1,\"r\":10,\"id\":37}],\"2734066259\":[{\"s\":2,\"r\":9,\"id\":34},{\"s\":1,\"r\":7,\"id\":25},{\"s\":4,\"r\":7,\"id\":28},{\"s\":4,\"r\":10,\"id\":40},{\"s\":1,\"r\":4,\"id\":13},{\"s\":1,\"r\":13,\"id\":49}],\"22483399774\":[{\"s\":1,\"r\":6,\"id\":21},{\"s\":4,\"r\":11,\"id\":44},{\"s\":3,\"r\":4,\"id\":15},{\"s\":4,\"r\":5,\"id\":20},{\"s\":3,\"r\":11,\"id\":43},{\"s\":4,\"r\":2,\"id\":8}],\"22233530764\":[{\"s\":3,\"r\":2,\"id\":7},{\"s\":1,\"r\":3,\"id\":9},{\"s\":4,\"r\":4,\"id\":16},{\"s\":3,\"r\":8,\"id\":31},{\"s\":3,\"r\":5,\"id\":19},{\"s\":1,\"r\":9,\"id\":33}]},\"hands_amount\":4,\"hands_order\":[\"22233530764\",\"2734066259\",\"2404048658\",\"22483399774\"],\"beat_confirmations\":{\"2404048658\":false,\"2734066259\":false,\"22233530764\":false,\"22483399774\":false},\"hand_size\":6,\"trump_suit\":3,\"table\":[[{\"s\":1,\"r\":2,\"id\":5},{\"s\":2,\"r\":6,\"id\":22}],[{\"s\":3,\"r\":12,\"id\":47},{\"s\":2,\"r\":2,\"id\":6}],[{\"s\":2,\"r\":10,\"id\":38},{\"s\":4,\"r\":12,\"id\":48}]]}}");

let HTMLHandsData = [];

const leaveLobbyAction = function () {
    disconnectWebsocket();
    navigate("/");
}

/**
 * Returns deck manager from game data
 * @param {any} gameData
 * @return {DeckManager}
 */
const getDeckManagerData = function (gameData) {
    return gameData.deck_manager
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
        for (let i = 0; i < item.cards.length; i++) {
            const card = item.cards[i];

            let cardGap = 20;
            let cardVGap = 10;
            let cardRotate = 5;
            let facing = false;

            if (item.id === "hand-container-1") {
                cardGap = 80;
                cardRotate = 0;
                cardVGap = 0;
                facing = true;
            }


            let localXOffset = Math.abs((item.cards.length - 1) / 2 - i) * cardVGap - 50;
            let localYOffset = ((item.cards.length - 1) / 2 - i) * cardGap;

            switch (item.dir) {
                case 1:
                    [localXOffset, localYOffset] = [localYOffset, localXOffset];
                    break;
                case 2:
                    [localXOffset, localYOffset] = [-localXOffset, localYOffset];
                    break;
                case 3:
                    [localXOffset, localYOffset] = [-localYOffset, -localXOffset];
                    break;
                case 4:
                    [localXOffset, localYOffset] = [localXOffset, -localYOffset];
                    break;
            }

            const offset = -75;

            const hOffset = item.dir === 1 ? offset : item.dir === 3 ? -offset : 0;
            const vOffset = item.dir === 2 ? offset : item.dir === 4 ? -offset : 0;

            const y = localYOffset + item.y - vOffset - 100 + facing * 150;
            const x = localXOffset + item.x - hOffset - 50;
            const rotation = (((item.cards.length - 1) / 2) - i) * cardRotate + [0, 90, 180, 270][item.dir - 1];

            cardRenderDataMap.set(card.id, {
                x, y, rotation, facing
            });
        }
    }
}

/**
 * Renders playing table by table data
 * @param {TableData} tableData
 * @return {string}
 */
const tableElements = function (tableData) {
    return `
    ${tableData.deck.map((card) => `<div class="card-container card-closed deck" id="card-${card.id}"></div>`).reverse().join("")}
    ${tableData.table.map(([bottom, top]) =>
            `<div class="card-container" id="card-${top.id}"></div> <div class="card-container table" id="card-${bottom.id}"></div>`
        ).join("")}
    ${tableData.discard.map((card) => `<div class="card-container card-closed discard" id="card-${card.id}"></div>`).join("")}
    <div class="table-container"></div>`
}

/**
 * Renders deck cards
 * @param {Card[]} deck
 * @param {{x: number, y: number}} position
 */
const renderDeck = function (deck, position) {
    let i = 0;

    const trump = deck.pop();
    const x = position.x + 20;
    const y = position.y;
    const facing = false;
    const rotation = 90;
    cardRenderDataMap.set(trump.id, { x, y, rotation, facing });
    for (const card of deck) {
        const scatter = 5;
        const x = position.x;
        const y = position.y + i * 0.5;
        const facing = false;
        const rotation = randomRange(-scatter, scatter);

        cardRenderDataMap.set(card.id, { x, y, rotation, facing });

        i += 1;
    }
}

/**
 * Renders table cards
 * @param {[Card, Card][]} table
 * @param {{x: number, y: number}} position
 */
const renderTable = function (table, position) {
    const rows = 2;
    const columns = Math.ceil(table.length / 2);

    const width = 500;
    const height = 400;

    for (let i = 0; i < table.length; i++) {
        let isBottom = true;
        for (const card of table[i]) {
            if (card == null) {
                continue;
            }

            const currentColumn = i % columns;
            const currentRow = Math.floor(i / rows);

            const scatter = 50;

            const x = position.x + Math.floor(width * (currentRow / rows))
            const y = position.y + Math.floor( height * (currentColumn / columns)) + isBottom * 30;
            const rotation = randomRange(-scatter, scatter);
            const facing = true;

            cardRenderDataMap.set(card.id, { x, y, rotation, facing });
            isBottom = false;
        }


    }
}

/**
 * Renders discard cards
 * @param {Card[]} discard
 * @param {{x: number, y: number}} position
 */
const renderDiscard = function (discard, position) {
    let i = 0;
    for (const card of discard) {
        const scatter = 50;
        const x = position.x + randomRange(-scatter, scatter);
        const y = position.y + i * 1.5 + randomRange(-scatter, scatter);
        const facing = false;
        const rotation = randomRange(-scatter, scatter);

        cardRenderDataMap.set(card.id, { x, y, rotation, facing });

        i += 1;
    }
}

/**
 * Manages renderer playing table elements properties
 * @param {TableData} tableData
 */
const postRenderTable = function (tableData) {
    const ww = window.innerWidth;
    const wh = window.innerHeight;

    const hGap = ww - 200 * 2;
    const vGap = (wh - 120) - 200 * 2;

    const xCenter = ww / 2;
    const yCenter = (wh - 120) / 2 - 120;

    const deckPosition = {
        x: 100,
        y: wh - 400
    };

    const tablePosition = {
        x: xCenter - 300,
        y: yCenter - 100
    }

    const discardPosition = {
        x: ww - 200,
        y: wh - 400
    };

    renderDeck(tableData.deck, deckPosition);
    renderTable(tableData.table, tablePosition);
    renderDiscard(tableData.discard, discardPosition);
}

/**
 * returns playing hands elements by hands data.
 * @param {HandData[]} handsData
 * @return {string}
 */
const handsElements = function(handsData) {
    const playerId = getUser()
    return handsData.map(hand => hand.cards.map(card =>
        `<div class="card-container card-closed ${hand.playerId === playerId ? "player" : "enemy"}" id="card-${card.id}"></div>`
    )).flat().join("") + "<div class='hand-container'></div>";
}

/**
 * place and position all cards on game table
 */
const positionAllCards = function () {
    const deck = gameData.deck_manager.full_deck;
    const playerId = getUser();

    console.log(Array.from(cardRenderDataMap.values()).length);

    for (const card of deck) {
        const {x, y, rotation, facing} = cardRenderDataMap.get(card.id);
        const HTMLCard = document.querySelector(`#card-${card.id}`);
        HTMLCard.style.left = `${x}px`;
        HTMLCard.style.top = `${y}px`;
        HTMLCard.style.rotate = `${rotation}deg`;

        if (HTMLCard.classList.contains("player")) {
            const handCards = gameData.deck_manager.hands[playerId];
            const cardsHTMLIds = handCards.map((card) =>  `card-${card.id}`);
            const i = handCards.indexOf(handCards.find(x => x.id === card.id));
            console.log(i);

            HTMLCard.addEventListener("mouseenter", () => {
                HTMLCard.style.transform = `translate(0, -50px)`;
                const leftNeighbors = cardsHTMLIds.slice(0, i).map(x => document.getElementById(x));;
                const rightNeighbors = cardsHTMLIds.slice(i + 1).map(x => document.getElementById(x));;
                for (const neighbor of leftNeighbors) {
                    neighbor.style.transform = "translateX(30px)";
                }
                for (const neighbor of rightNeighbors) {
                    neighbor.style.transform = "translateX(-30px)";
                }
            })

            HTMLCard.addEventListener("mouseleave", () => {
                HTMLCard.style.transform = `translate(0, 0)`;
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

    console.log(gameData);

    const deckManager = getDeckManagerData(gameData);

    const handsData = fetchHandDataOnMount(
        deckManager.hands,
        deckManager.hands_order,
        getUser()
    );

    const tableData = {
        deck: deckManager.deck,
        table: deckManager.table,
        discard: deckManager.discard,
        trumpSuit: deckManager.trump_suit
    }

    document.addEventListener(PAGE_RENDER_EVENT_ID, () => {
        if (document.querySelector(".table-container")) {
            postRenderTable(tableData);
            console.log(Array.from(cardRenderDataMap.values()).length);
            postRenderHands(handsData);
            console.log(Array.from(cardRenderDataMap.values()).length);
            triggerRenderPageId();
        }

        if (document.querySelector(".card-container")) {
            positionAllCards();
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
        ${handsElements(handsData)}
         ${tableElements(tableData)}}
    </div>
</div>`
}

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /leave-button/.test(className): leaveLobbyAction(); break;
    }
});

/**
 * updates game data
 * @param newGameData
 */
export const updateGameData = function (newGameData) {
    gameData = newGameData;
}

export const CardType = {
    Player: "player",
    Enemy: "enemy",
    Deck: "deck",
    Discard: "discard",
    Table: "table"
}

/**
 * Moves given card into new position.
 * @param {number} cardId
 * @param {$Values<CardType>} cardType
 * @param {number | string} index
 */
export const moveCard = function (cardId, cardType, index) {

}
