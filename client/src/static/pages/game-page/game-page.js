import {templates} from "./table-templates.js"
import {navigate, PAGE_RENDER_EVENT_ID, randomRange, triggerRenderPageId} from "../../utils/index.js";
import {disconnectWebsocket} from "../../websocket/index.js";
import {getUser} from "../../state/index.js";
import {wsGameBeat, wsGameConfirmPass, wsGameInitTurn, wsGameTake, wsGameToss} from "../../websocket/handle-game.js";

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

let gameData = {};

//
// let gameData = JSON.parse("{\"id\":\"GAME4273841987\",\"status\":\"Start\",\"participant_ids\":[\"22233530764\",\"2404048658\",\"2734066259\",\"22483399774\"],\"attacker_player_id\":\"2734066259\",\"target_player_id\":\"2404048658\",\"next_player_id\":\"22233530764\",\"turn_queue\":[],\"deck_manager\":{\"full_deck\":[{\"s\":1,\"r\":1,\"id\":1},{\"s\":2,\"r\":1,\"id\":2},{\"s\":3,\"r\":1,\"id\":3},{\"s\":4,\"r\":1,\"id\":4},{\"s\":1,\"r\":2,\"id\":5},{\"s\":2,\"r\":2,\"id\":6},{\"s\":3,\"r\":2,\"id\":7},{\"s\":4,\"r\":2,\"id\":8},{\"s\":1,\"r\":3,\"id\":9},{\"s\":2,\"r\":3,\"id\":10},{\"s\":3,\"r\":3,\"id\":11},{\"s\":4,\"r\":3,\"id\":12},{\"s\":1,\"r\":4,\"id\":13},{\"s\":2,\"r\":4,\"id\":14},{\"s\":3,\"r\":4,\"id\":15},{\"s\":4,\"r\":4,\"id\":16},{\"s\":1,\"r\":5,\"id\":17},{\"s\":2,\"r\":5,\"id\":18},{\"s\":3,\"r\":5,\"id\":19},{\"s\":4,\"r\":5,\"id\":20},{\"s\":1,\"r\":6,\"id\":21},{\"s\":2,\"r\":6,\"id\":22},{\"s\":3,\"r\":6,\"id\":23},{\"s\":4,\"r\":6,\"id\":24},{\"s\":1,\"r\":7,\"id\":25},{\"s\":2,\"r\":7,\"id\":26},{\"s\":3,\"r\":7,\"id\":27},{\"s\":4,\"r\":7,\"id\":28},{\"s\":1,\"r\":8,\"id\":29},{\"s\":2,\"r\":8,\"id\":30},{\"s\":3,\"r\":8,\"id\":31},{\"s\":4,\"r\":8,\"id\":32},{\"s\":1,\"r\":9,\"id\":33},{\"s\":2,\"r\":9,\"id\":34},{\"s\":3,\"r\":9,\"id\":35},{\"s\":4,\"r\":9,\"id\":36},{\"s\":1,\"r\":10,\"id\":37},{\"s\":2,\"r\":10,\"id\":38},{\"s\":3,\"r\":10,\"id\":39},{\"s\":4,\"r\":10,\"id\":40},{\"s\":1,\"r\":11,\"id\":41},{\"s\":2,\"r\":11,\"id\":42},{\"s\":3,\"r\":11,\"id\":43},{\"s\":4,\"r\":11,\"id\":44},{\"s\":1,\"r\":12,\"id\":45},{\"s\":2,\"r\":12,\"id\":46},{\"s\":3,\"r\":12,\"id\":47},{\"s\":4,\"r\":12,\"id\":48},{\"s\":1,\"r\":13,\"id\":49},{\"s\":2,\"r\":13,\"id\":50},{\"s\":3,\"r\":13,\"id\":51},{\"s\":4,\"r\":13,\"id\":52}],\"deck\":[{\"s\":1,\"r\":1,\"id\":1},{\"s\":4,\"r\":6,\"id\":24},{\"s\":2,\"r\":3,\"id\":10},{\"s\":3,\"r\":10,\"id\":39},{\"s\":4,\"r\":13,\"id\":52},{\"s\":3,\"r\":13,\"id\":51},{\"s\":4,\"r\":3,\"id\":12},{\"s\":2,\"r\":13,\"id\":50},{\"s\":3,\"r\":6,\"id\":23},{\"s\":4,\"r\":1,\"id\":4},{\"s\":2,\"r\":7,\"id\":26},{\"s\":2,\"r\":4,\"id\":14},{\"s\":2,\"r\":11,\"id\":42},{\"s\":1,\"r\":8,\"id\":29},{\"s\":4,\"r\":9,\"id\":36},{\"s\":2,\"r\":1,\"id\":2},{\"s\":1,\"r\":5,\"id\":17},{\"s\":4,\"r\":8,\"id\":32},{\"s\":3,\"r\":9,\"id\":35}],\"discard\":[{\"s\":3,\"r\":7,\"id\":27},{\"s\":2,\"r\":12,\"id\":46},{\"s\":3,\"r\":3,\"id\":11}],\"hands\":{\"2404048658\":[{\"s\":2,\"r\":8,\"id\":30},{\"s\":1,\"r\":12,\"id\":45},{\"s\":3,\"r\":1,\"id\":3},{\"s\":1,\"r\":11,\"id\":41},{\"s\":2,\"r\":5,\"id\":18},{\"s\":1,\"r\":10,\"id\":37}],\"2734066259\":[{\"s\":2,\"r\":9,\"id\":34},{\"s\":1,\"r\":7,\"id\":25},{\"s\":4,\"r\":7,\"id\":28},{\"s\":4,\"r\":10,\"id\":40},{\"s\":1,\"r\":4,\"id\":13},{\"s\":1,\"r\":13,\"id\":49}],\"22483399774\":[{\"s\":1,\"r\":6,\"id\":21},{\"s\":4,\"r\":11,\"id\":44},{\"s\":3,\"r\":4,\"id\":15},{\"s\":4,\"r\":5,\"id\":20},{\"s\":3,\"r\":11,\"id\":43},{\"s\":4,\"r\":2,\"id\":8}],\"22233530764\":[{\"s\":3,\"r\":2,\"id\":7},{\"s\":1,\"r\":3,\"id\":9},{\"s\":4,\"r\":4,\"id\":16},{\"s\":3,\"r\":8,\"id\":31},{\"s\":3,\"r\":5,\"id\":19},{\"s\":1,\"r\":9,\"id\":33}]},\"hands_amount\":4,\"hands_order\":[\"22233530764\",\"2734066259\",\"2404048658\",\"22483399774\"],\"beat_confirmations\":{\"2404048658\":false,\"2734066259\":false,\"22233530764\":false,\"22483399774\":false},\"hand_size\":6,\"trump_suit\":3,\"table\":[[{\"s\":1,\"r\":2,\"id\":5},{\"s\":2,\"r\":6,\"id\":22}],[{\"s\":3,\"r\":12,\"id\":47},{\"s\":2,\"r\":2,\"id\":6}],[{\"s\":2,\"r\":10,\"id\":38},{\"s\":4,\"r\":12,\"id\":48}]]}}");
//
// console.log(gameData);
//
// window.lobbyData = {
//     player_list: gameData.deck_manager.hands_order.map((x, i) => ({id: x, name: `Player${i + 1}`}))
// }

/**
 * @type {HandData[]}
 */
let handsData = [];

const ww = window.innerWidth;
const wh = window.innerHeight;

const xCenter = ww / 2;
const yCenter = (wh - 120) / 2 - 120;

/**
 * @type {Card | null}
 */
let selectedCard = null;

const listeners = [];

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

    const template = templates[order.length - 1];
    const templateLen = template.length;
    handsData = new Array(order.length).fill(null);
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
                const handX = j * ww / (templateLen - 1) + hGap / 10 - hOffset;
                const handY = i * wh / (templateLen - 1) + vGap - vOffset - headerSize - hOffset * 0.5;

                handsData[num - 1] = {
                    x: handX,
                    y: handY,
                    dir: dir,
                    id: `hand-container-${num}`,
                    playerId: order[num - 1],
                    cards: hands[order[num - 1]].toSorted(x => x.s * 14 + x.r)
                };
            }
        }
    }

    return handsData;
}

/**
 * Set player tag name.
 * @param id
 * @param HTMLTag
 * @return {Promise<void>}
 */
const setPlayerName = async function (id, HTMLTag) {
    HTMLTag.innerHTML = window.lobbyData.player_list.find(x => x.id === id).name;
}

/**
 * Draws and manages player hands styles
 * @param {HandData[]} handsData
 */
const postRenderHands = function () {
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
            const zIndex = item.cards.length - i;

            cardRenderDataMap.set(card.id, {
                x, y, rotation, facing, zIndex
            });
        }
    }
}

/**
 * renders ui: player labels, buttons, messages
 */
const renderUi = function () {
    const clientId = getUser();

    const msg = document.querySelector(".msg");
    msg.innerHTML = "";
    msg.style.left = `${xCenter - 50}px`;
    msg.style.top = `${yCenter + 280}px`;

    const takeBtn = document.querySelector("#take-button");
    takeBtn.style.left = `${xCenter - 50}px`;
    takeBtn.style.top = `${yCenter + 300}px`;
    takeBtn.classList.add("hidden");
    takeBtn.removeEventListeners("click");
    takeBtn.addEventListener("click", () => {
        wsGameTake();
    })

    const passBtn = document.querySelector("#pass-button");
    passBtn.style.left = `${xCenter - 50}px`;
    passBtn.style.top = `${yCenter +300}px`;
    passBtn.classList.add("hidden");
    passBtn.removeEventListeners("click");
    passBtn.addEventListener("click", () => {
        for (const id of gameData.deck_manager.hands_order) {
            wsGameConfirmPass(id);
        }
    })

    if (gameData.target_player_id !== clientId && gameData.deck_manager.table.length !== 0) {
        passBtn.classList.remove("hidden");
    }

    if (gameData.target_player_id === clientId && gameData.deck_manager.table.length !== 0) {
        takeBtn.classList.remove("hidden");
    }

    for (const hand of handsData) {
        const {x, y, playerId, dir} = hand;
        const offset = 75;
        const margin = 35;
        const hOffset = dir === 1 ? offset : dir === 3 ? -offset : 0;
        const vOffset = dir === 2 ? offset : dir === 4 ? -offset : 0;
        const vMargin = dir === 3 ? margin * 0.8 : dir === 1 ? -margin * 0.8 : 0;
        const hMargin = dir === 2 ? margin : dir === 4 ? -margin : 0;

        const HTMLPlayerTag = document.querySelector(`#player-${playerId}`);

        if (playerId === clientId) {
            HTMLPlayerTag.classList.add("hidden");
            if (playerId === gameData.attacker_player_id) {
                msg.innerHTML = "your turn!";
            }
        }

        if (playerId === gameData.attacker_player_id) {
            HTMLPlayerTag.style.border = "3px solid red";
        }

        HTMLPlayerTag.style.left = `${x - 50 + hOffset + hMargin}px`;
        HTMLPlayerTag.style.top = `${y - 75 + vOffset + vMargin}px`;
        setPlayerName(playerId, HTMLPlayerTag);
    }
}

/**
 * Renders playing table by table data
 * @param {TableData} tableData
 * @return {string}
 */
const tableElements = function (tableData) {
    return `
    ${tableData.deck.map((card, index) => 
        `<div class="card-container ${index !== tableData.deck.length - 1 ? "card-closed" : ""} deck" id="card-${card.id}"></div>`
    ).reverse().join("")}
    ${tableData.table.map(([bottom, top]) =>
        `<div class="card-container table" id="card-${top.id}"></div> <div class="card-container table" id="card-${bottom.id}"></div>`
    ).join("")} 
    ${tableData.discard.map((card) => 
        `<div class="card-container card-closed discard" id="card-${card.id}"></div>`
    ).join("")}
    <div class="table-container"></div>`
}

/**
 * Renders deck cards
 * @param {Card[]} deck
 */
const renderDeck = function (deck) {
    let i = 0;

    const trump = deck.pop();
    const x = deckPosition.x + 20;
    const y = deckPosition.y;
    const facing = false;
    const rotation = 90;
    const zIndex = 1;
    cardRenderDataMap.set(trump.id, { x, y, rotation, facing, zIndex });
    for (const card of deck) {
        const scatter = 5;
        const x = deckPosition.x;
        const y = deckPosition.y + i * 0.5;
        const facing = false;
        const rotation = randomRange(-scatter, scatter);
        const zIndex = 1;

        cardRenderDataMap.set(card.id, { x, y, rotation, facing, zIndex });

        i += 1;
    }
}

/**
 * Renders table cards
 * @param {[Card, Card][]} table
 */
const renderTable = function (table) {
    for (let i = 0; i < table.length; i++) {
        let isBottom = true;
        for (const card of table[i]) {
            if (card == null) {
                isBottom = false;
                continue;
            }

            if (cardRenderDataMap.has(card.id)) {
                isBottom = false;
                continue;
            }

            const width = 500;
            const height = 400;

            const rows = 2;
            const columns = Math.ceil(table.length / 2);

            const currentColumn = i % columns;
            const currentRow = Math.floor(i / rows);

            const scatter = 50;

            const x = tablePosition.x + Math.floor(width * (currentRow / rows))
            const y = tablePosition.y + Math.floor( height * (currentColumn / columns)) + isBottom * 30;
            const rotation = randomRange(-scatter, scatter);
            const facing = true;
            const zIndex = isBottom ? 60 : 61;

            cardRenderDataMap.set(card.id, { x, y, rotation, facing, zIndex });

            isBottom = false;
        }
    }
}

let discardVOffset = 0;

/**
 * moves card to discard
 * @param {Card} card
 */
const moveCardToDiscard = function (card) {
    const scatter = 50;
    const x = discardPosition.x + randomRange(-scatter, scatter);
    const y = discardPosition.y + discardVOffset * 1.5 + randomRange(-scatter, scatter);
    const facing = false;
    const rotation = randomRange(-scatter, scatter);
    const zIndex = discardVOffset;

    cardRenderDataMap.set(card.id, { x, y, rotation, facing, zIndex });
    discardVOffset += 1;
}

/**
 * Renders discard cards
 * @param {Card[]} discard
 */
const renderDiscard = function (discard) {
    for (const card of discard) {
        moveCardToDiscard(card);
    }
}

/**
 * Manages renderer playing table elements properties
 * @param {TableData} tableData
 */
const postRenderTable = function (tableData) {
    renderDeck(tableData.deck);
    renderTable(tableData.table);
    renderDiscard(tableData.discard);
}

/**
 * returns playing hands elements by hands data.
 * @param {HandData[]} handsData
 * @return {string}
 */
const handsElements = function() {
    const playerId = getUser()
    return handsData.map(hand => hand.cards.map(card =>
        `<div 
class="card-container ${hand.playerId === playerId ? "player client" : "card-closed player"}" 
owner="${hand.playerId}" 
id="card-${card.id}"></div>`
    )).flat().reverse().join("") + "<div class='hand-container'></div>";
}

/**
 * place and position all cards on game table
 */
const positionAllCards = function () {
    const deck = gameData.deck_manager.full_deck;
    const playerId = getUser();

    for (const card of deck) {
        const {x, y, rotation, facing, zIndex} = cardRenderDataMap.get(card.id);
        const HTMLCard = document.querySelector(`#card-${card.id}`);
        HTMLCard.style.left = `${x}px`;
        HTMLCard.style.top = `${y}px`;
        HTMLCard.style.rotate = `${rotation}deg`;
        HTMLCard.style.zIndex = `${zIndex}`;
        HTMLCard.style.transform = `translate(0, 0)`;
        HTMLCard.style.border = "1px solid black";

        if (!HTMLCard.style.backgroundImage) {
            if (!HTMLCard.classList.contains("card-closed")) {
                HTMLCard.style.backgroundImage = `url("/assets/cards/card-${card.s}-${card.r}.png")`;
            } else {
                HTMLCard.style.backgroundImage = `url("/assets/cards/card-closed.png")`;
            }
        }

        HTMLCard.removeEventListeners("click");
        HTMLCard.removeEventListeners("mouseenter");
        HTMLCard.removeEventListeners("mouseleave");

        HTMLCard.addEventListener("click", (event) => {
            if (HTMLCard.classList.contains("client")) {
                const hand = handsData.find(x => x.playerId === playerId).cards;
                const HTMLCards = hand.map((card) => document.querySelector(`#card-${card.id}`));

                if (HTMLCard.classList.contains("selected")) {
                    HTMLCard.classList.remove("selected");
                    HTMLCard.style.transform = "translate(0, 0)";
                    HTMLCard.style.border = "1px solid black";
                    selectedCard = null;
                } else {
                    HTMLCards.forEach(item => {
                        item.classList.remove("selected");
                        item.style.transform = "translate(0, 0)";
                        item.style.border = "1px solid black";
                    });
                    HTMLCard.classList.add("selected");
                    HTMLCard.style.transform = "translate(0, -60px)";
                    HTMLCard.style.border = "3px solid #ff00ff";
                    selectedCard = card;
                }
            }

            if (HTMLCard.classList.contains("table")) {
                if (selectedCard) {
                    //const targetCard = gameData.deck_manager.full_deck.find(x => x.id === Number(event.target.id.split("-")[1]));

                    wsGameBeat(selectedCard, card, getUser());
                }
            }
        });

        HTMLCard.addEventListener("click", () => {

        });

        if (HTMLCard.classList.contains("client")) {
            const hand = handsData.find(x => x.playerId === playerId).cards;

            const HTMLHands = hand.map((card) =>  `card-${card.id}`);

            const i = hand.indexOf(hand.find(x => x.id === card.id));

            const leftNeighbors = HTMLHands.slice(0, i).map(x => document.getElementById(x));

            const rightNeighbors = HTMLHands.slice(i + 1).map(x => document.getElementById(x));

            HTMLCard.addEventListener("mouseenter", () => {
                if (selectedCard != null) {
                    return;
                }
                HTMLCard.style.transform = `translate(0, -50px)`;

                for (const neighbor of leftNeighbors) {
                    neighbor.style.transform = "translateX(30px)";
                }
                for (const neighbor of rightNeighbors) {
                    neighbor.style.transform = "translateX(-30px)";
                }
            });

            HTMLCard.addEventListener("mouseleave", () => {
                if (selectedCard != null) {
                    return;
                }
                HTMLCard.style.transform = `translate(0, 0)`;

                for (const neighbor of leftNeighbors) {
                    neighbor.style.transform = "translate(0, 0)";
                }
                for (const neighbor of rightNeighbors) {
                    neighbor.style.transform = "translate(0, 0)";
                }
            });
        }
    }
    renderUi();
}

/**
 * returns ui elements
 * @return {string}
 */
const uiElements = function () {
    return `
${gameData.deck_manager.hands_order.map(id => `<div class="player-nickname" id="player-${id}"></div>`).join("")}
${gameData.deck_manager.hands_order.map(id => `<div class="attacker-tag" id="attacker-${id}">attacker</div>`).join("")}
<button class="game-button" id="take-button">Take</button>
<button class="game-button" id="pass-button">Pass</button>
<div class="msg"></div>`
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

    gameData = lobbyData.game;


    const deckManager = getDeckManagerData(gameData);

    handsData = fetchHandDataOnMount(
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

    const render = () => {
        if (document.querySelector(".table-container")) {
            const table = document.querySelector(".table-container");
            const w = 600;
            const h = 400;
            table.style.width = `${w}px`;
            table.style.height = `${h}px`;
            table.style.left = `${(ww - w) / 2}px`;
            table.style.top = `${(wh - h) / 2 - 100}px`;

            table.addEventListener("click", () => {
                if (!selectedCard) {
                    return;
                }
                if (gameData.status === "Turn") {
                    wsGameToss(selectedCard, getUser());
                } else {
                    wsGameInitTurn(selectedCard, getUser());
                }
            })

            postRenderTable(tableData);
            postRenderHands();
            renderUi();
            triggerRenderPageId();
        }

        if (document.querySelector(".card-container")) {
            positionAllCards();
        }
    }

    document.addEventListener(PAGE_RENDER_EVENT_ID, render);

    return `
<div class="game-page-wrapper page-wrapper">
    <div class="lobby-header">
        <div class="title"> WEB DURAK </div>
        <button class="leave-button"> LEAVE </button>
    </div>
    <div class="game-container">
        <div class="ui-container">${uiElements()}</div>
        <div class="cards">${handsElements()} ${tableElements(tableData)}}</div>
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
    Deck: "deck",
    Discard: "discard",
    Table: "table",
    Nobody: "Nobody"
}

/**
 * adds card from hand and re-calculates card coordinates
 * @param {string} playerId
 * @param {Card} card
 */
const addCardToHand = function (playerId, card) {
    const index = handsData.indexOf(handsData.find(x => x.playerId === playerId));
    handsData[index].cards.push(card);
    handsData[index].cards = handsData[index].cards.toSorted(x => x.s * 14 + x.r).reverse();
    gameData.deck_manager.hands[playerId].push(card);
    gameData.deck_manager.hands[playerId] = gameData.deck_manager.hands[playerId].toSorted(x => x.s * 14 + x.r).reverse();
}

/**
 * adds card from hand and re-calculates card coordinates
 * @param {string} playerId
 * @param {Card} card
 * @param {$Values<CardType>} whereTo
 */
const removeCardFromHand = function (playerId, card, whereTo) {
    const index = handsData.indexOf(handsData.find(x => x.playerId === playerId));
    const newHand = handsData[index].cards.filter(x => x.id !== card.id);
    handsData[index].cards = newHand;
    gameData.deck_manager.hands[playerId] = newHand;
    cardRenderDataMap.delete(card.id);
}

/**
 * Adds card to table client and updates gameData
 * @param card
 * @param index
 */
const addCardToTableOfGameData = function (card, index) {
    if (gameData.deck_manager.table.length > index) {
        gameData.deck_manager.table[index][1] = card;
    } else {
        gameData.deck_manager.table.push([card, null]);
    }
}

/**
 * Removes card from table client and updates gameData
 * @param {Card} card
 */
const removeCardFromTableOfGameData = function (card) {
    for (let i = 0; i < gameData.deck_manager.table.length; i++) {
        let currentPair = gameData.deck_manager.table[i];
        if (currentPair[0]) {
            if (currentPair[0].id === card.id) {
                gameData.deck_manager.table[i][0] = null;
            }
        }else if (currentPair[1]) {
            if (currentPair[1].id === card.id) {
                gameData.deck_manager.table[i][1] = null;
            }
        }
        else if (i === gameData.deck_manager.table.length - 1) {
            gameData.deck_manager.table.pop();
        }
    }
}

/**
 * removes card from deck
 * @param {Card} card
 */
const removeCardFromDeck = function (card ) {
    cardRenderDataMap.delete(card.id);
}


/**
 * Moves given card into new position.
 * @param {Card} card
 * @param {$Values<CardType>} whereTo
 * @param {number | string} indexFrom
 * @param {number | string} indexTo
 */
export const moveCard = function (card, whereTo, indexFrom = 0, indexTo = 0) {
    const clientId = getUser();

    const HTMLCard = document.querySelector(`#card-${card.id}`);

    const whereFrom = Array.from(HTMLCard.classList.values()).filter(x => Object.values(CardType).includes(x))[0];

    if (whereTo === CardType.Discard) {
        if (whereFrom === CardType.Table) {
            HTMLCard.classList.add("card-closed");
            HTMLCard.classList.add("discard");
            HTMLCard.classList.remove("table");
            HTMLCard.style.backgroundImage = "";
            HTMLCard.style.zIndex = discardVOffset;
            moveCardToDiscard(card);
        }
    } else if (whereTo === CardType.Table) {
        if (whereFrom === CardType.Player) {
            HTMLCard.classList.add("table");
            HTMLCard.classList.remove("player");
            if (!HTMLCard.classList.contains("client")) {
                HTMLCard.classList.remove("card-closed");
                HTMLCard.style.backgroundImage = "";
            }
            HTMLCard.classList.remove("client");
            HTMLCard.style.transform = "";
            removeCardFromHand(HTMLCard.getAttribute("owner"), card, whereTo);
            HTMLCard.removeAttribute("owner");
            addCardToTableOfGameData(card, indexTo);
            renderTable(gameData.deck_manager.table);
            postRenderHands();
        }
    } else if (whereTo === CardType.Player) {
        console.log(whereTo, whereFrom, indexTo, indexFrom, clientId);

        if (whereFrom === CardType.Table) {
            HTMLCard.classList.add("player");
            HTMLCard.classList.remove("table");
            if (indexTo !== clientId) {
                HTMLCard.classList.add("card-closed");
                HTMLCard.style.backgroundImage = "";
            } else {
                HTMLCard.classList.add("client");
            }
            HTMLCard.setAttribute("owner", indexTo);
            removeCardFromTableOfGameData(card);
            addCardToHand(indexTo, card);
            postRenderHands();
        } else if (whereFrom === CardType.Deck) {
            HTMLCard.classList.add("player");
            HTMLCard.classList.remove("deck");
            if (indexTo === clientId) {
                HTMLCard.classList.add("client");
                HTMLCard.classList.remove("card-closed");
                HTMLCard.style.backgroundImage = "";
            }
            HTMLCard.setAttribute("owner", indexTo);
            removeCardFromDeck(card);
            addCardToHand(indexTo, card);
            postRenderHands();
        }
    }
    positionAllCards();
}
