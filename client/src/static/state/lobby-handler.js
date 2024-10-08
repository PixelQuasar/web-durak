const LOBBY_ID = "lobby"

/**
 * get lobby id from local storage
 * @returns {string}
 */
export const getLobby = function () {
    return localStorage.getItem(LOBBY_ID);
}

/**
 * Save lobby id to local storage
 * @param {string} id
 */
export const setLobby = function (id) {
    localStorage.setItem(LOBBY_ID, id);
}

/**
 * Get lobby info from server REST endpoint
 * @param {string} id
 * @returns {Promise<any>}
 */
export const getLobbyQuery = async function (id) {
    const response = await fetch(`${process.env.SERVER_URL}/lobby/${id}`);

    return await response.json();
}

/**
 * Get all lobbies from server
 * @return {Promise<Array<any>>}
 */
export const getLobbiesListQuery = async function () {
    const response = await fetch(`${process.env.SERVER_URL}/lobby`);

    return await response.json();
}

/**
 * Get lobby scoreboard
 * @param {string} id
 * @return {Promise<any>}
 */
export const scoreBoardQuery = async function (id) {
    const response = await fetch(`${process.env.SERVER_URL}/lobby/scoreboard/${id}`);

    return await response.json();
}