const demoAction = function () {
    window.socket = new WebSocket("ws://localhost:3500/ws");

    window.socket.addEventListener("open", (event) => {
        //socket.send("Hello Server!");
        console.log("Connection opened.");
    });

    window.socket.addEventListener("close", (event) => {
        console.log("Connection closed.");
    })

    window.socket.addEventListener("message", (event) => {
        console.log("Message from server ", event.data);
    });
};

const wsAction = function () {
    if (!window.socket) return;

    const content = JSON.stringify({
        req_type: "LobbyJoin",
        sender_id: "PLAYER1034438148",
        lobby_id: "LOBBY2458306941",
        content: JSON.stringify({public: true})
    });

    console.log(content);

    window.socket.send(content);
};

export const Homepage = function () {
    return `
<div class="homepage-wrapper">
    Homepage
    <button class="demo-button">hello</button>
    <button class="demo-ws-button">websocket</button>
</div>
`
};

document.querySelector("body").addEventListener("click", (event) => {
    switch (event.target.className) {
        case "demo-button": demoAction(); break;
        case "demo-ws-button": wsAction(); break;
        default: break;
    }
});
