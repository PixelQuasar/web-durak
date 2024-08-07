import {handleServerMessage} from "./handle-server-message.js";

export const initWebsocketConnection = function () {
    if (window.websocket) return;
    window.websocket = new WebSocket(`${process.env.WEBSOCKET_URL}/ws`);

    window.websocket.addEventListener("open", (event) => {
        // handle connection open
        console.log("Connection opened.");
    });

    window.websocket.addEventListener("message", async (event) => {
        if (event.data[0] === "{") {
            await handleServerMessage(event.data);
        }
    });

    window.websocket.addEventListener("close", (event) => {
        // handle connection close
        console.log("Connection closed.");
        window.websocket = null;
    });
}

export const disconnectWebsocket = function () {
    if (!window.websocket) return;

    window.websocket.close();
}
