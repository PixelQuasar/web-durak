const demoAction = function () {
    window.socket = new WebSocket("ws://localhost:3500/ws");

    window.socket.addEventListener("open", (event) => {
        socket.send("Hello Server!");
    });

    window.socket.addEventListener("message", (event) => {
        console.log("Message from server ", event.data);
    });
};

const wsAction = function () {
    if (!window.socket) return;
    window.socket.send("Hello Server!");
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
