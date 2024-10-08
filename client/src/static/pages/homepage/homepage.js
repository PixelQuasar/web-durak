import {navigate} from "../../utils/index.js";

/**
 * Returns WEB DURAK index page.
 * @returns {string}
 */
export const Homepage = function () {
    return `
<div class="homepage-wrapper page-wrapper">
    <h1 class="page-title">WEB DUR<span>A</span>K</h1>
    <div class="button-wrapper">
        <button class="menu-button join-lobby-button">JOIN LOBBY</button>
        <button class="menu-button create-lobby-button">CREATE LOBBY</button>
    </div>
</div>`
};

document.querySelector("body").addEventListener("click", (event) => {
    const className = event.target.className
    switch (true) {
        case /join-lobby-button/.test(className): navigate("/join-lobby"); break;
        case /create-lobby-button/.test(className): navigate("/create-lobby"); break;
    }
});
