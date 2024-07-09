const joinLobbyAction = function () {
    console.log("Join lobby");
}

export const JoinLobby = function () {
    return `
<div class="join-lobby-wrapper">
   <div class="title">
        WEB DURAK
    </div>
    <div class="subtitle">
        The layout is not final.
    </div>
    
    <input type="text" class="join-lobby-textbox" placeholder="enter lobby code"/>
   <button class="join-button">JOIN LOBBY</button>
   <div class="error-msg"></div>
</div>
    `
}

document.querySelector("body").addEventListener("click", (event) => {
    switch (event.target.className) {
        case "join-button": joinLobbyAction(); break;
        default: break;
    }
})

document.querySelector("body").addEventListener("keypress", (event) => {
    if (event.key !== "enter") return;

    switch (event.target.className) {
        case "join-lobby-textbox": joinLobbyAction(); break;
        default: break;
    }
})
