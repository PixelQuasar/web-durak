import {handleServerMessage} from "./handle-server-message.js";

/**
 * Initiates websocket connection for further game.
 * @param {Function} callback
 */
export const initWebsocketConnection = function (callback = () => {}) {
    if (window.websocket) {
        return callback();
    }

    window.websocket = new WebSocket(`${process.env.WEBSOCKET_URL}/ws`);

    window.websocket.addEventListener("open", (event) => {
        // handle connection open
        console.log("Connection opened.");
        callback();
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
