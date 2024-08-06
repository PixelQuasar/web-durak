const USER_ID = "user"

/**
 * Get user id from local storage.
 * @returns {string}
 */
export const getUser = function () {
    return localStorage.getItem(USER_ID);
}

/**
 * Save user id to local storage.
 * @param id
 */
export const setUser = function (id) {
    return localStorage.setItem(USER_ID, id);
}

/**
 * Get user info from server REST endpoint.
 * @param {string} name
 * @returns {Promise<any>}
 */
export const createUserQuery = async function (name) {
    const response = await fetch(`${process.env.SERVER_URL}/player`, {
        method: "POST",
        headers: {
            "Accept": "application/json",
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ name: name }),
    });

    return await response.json();
}
