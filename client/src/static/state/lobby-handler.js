const LOBBY_ID = "lobby"

export const getLobby = function () {
    return localStorage.getItem(LOBBY_ID);
}

export const setLobby = function (id) {
    localStorage.setItem(LOBBY_ID, id);
}
