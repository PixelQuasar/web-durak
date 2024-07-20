const USER_ID = "user"

export const getUser = function () {
    return localStorage.getItem(USER_ID);
}

export const setUser = function (id) {
    return localStorage.setItem(USER_ID, id);
}

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
