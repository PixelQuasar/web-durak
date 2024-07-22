const LOBBY_ID = "lobby"

export const getLobby = function () {
    return localStorage.getItem(LOBBY_ID);
}

export const setLobby = function (id) {
    localStorage.setItem(LOBBY_ID, id);
}

export const getLobbyQuery = async function (id) {
    const response = await fetch(`${process.env.SERVER_URL}/lobby/${id}`);

    return await response.json();
}
