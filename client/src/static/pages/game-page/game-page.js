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

</div>`
}

