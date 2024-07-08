const createLobbyAction = function () {
    console.log("CREATE LOBBY");
}


export const CreateLobby = function () {
    return `
<div class="create-lobby-wrapper">
   <div class="title">
        WEB DURAK
    </div>
    <div class="subtitle">
        The layout is not final.
    </div>
    
   <button class="create-button">CREATE LOBBY</button>
</div>
    `
}

document.querySelector("body").addEventListener("click", (event) => {
    switch (event.target.className) {
        case "create-button": createLobbyAction(); break;
        default: break;
    }
})
