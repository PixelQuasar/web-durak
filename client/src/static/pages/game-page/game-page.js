/**
 * A playing card.
 * @typedef {Object} Card
 * @property {number} r - rank
 * @property {number} s - suit
 */

/**
 * @param {Card[]} hands
 * @return {string}
 */
const renderHands = function(hands) {

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

    return `
<div class="game-page-wrapper page-wrapper">
    ${JSON.stringify(lobbyData.game)}
</div>`
}

