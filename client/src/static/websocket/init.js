import {handleServerMessage} from "./handle-server-message.js";

export const initWebsocketConnection = function () {
    if (window.websocket) return;
    window.websocket = new WebSocket(`${process.env.WEBSOCKET_URL}/ws`);

    window.websocket.addEventListener("open", (event) => {
        // handle connection open
        console.log("Connection opened.");
    });

    window.websocket.addEventListener("message", (event) => {
        console.log("WebSocket message: ", event.data);
        if (event.data[0] === "{") {
            handleServerMessage(event.data);
        }
    });

    window.websocket.addEventListener("close", (event) => {
        // handle connection close
        console.log("Connection closed.");
    });
}
