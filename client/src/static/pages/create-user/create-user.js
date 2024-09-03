import {createUserQuery, setUser} from "../../state/index.js";
import {navigate} from "../../utils/index.js";

const createUserAction = async function () {
    const userName = document.querySelector("#user-name-input").value;

    const userData = await createUserQuery(userName);

    setUser(userData.id);

    navigate("/");
}

/**
 * Returns page where user identifies itself.
 * @returns {string}
 */
export const CreateUser = function () {
    return `
<div class="create-lobby-wrapper page-wrapper">
    <h1 class="page-title">WEB DUR<span>A</span>K</h1>
    <input type="text" id="user-name-input" class="large-textbox" placeholder="Type your name here">
    <button class="create-user-button">CREATE USER</button>
</div>`
}

document.querySelector("body").addEventListener("click",  (event) => {
    switch (event.target.className) {
        case "create-user-button": createUserAction(); break;
    }
});

document.querySelector("body").addEventListener("keypress", (event) => {
    if (event.target.id === "user-name-input" && event.key === "Enter") {
        createUserAction();
    }
});
