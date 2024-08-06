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
   <div class="title">
        WEB DURAK
    </div>
    <div class="subtitle">
        The layout is not final.
    </div>
    
    <input type="text" id="user-name-input" class="large-textbox" placeholder="Type your name here">
    <button class="create-user-button">CREATE USER</button>
</div>
    `
}

document.querySelector("body").addEventListener("click", async (event) => {
    switch (event.target.className) {
        case "create-user-button": await createUserAction(); break;
    }
});
