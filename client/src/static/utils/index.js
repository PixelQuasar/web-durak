export const PAGE_CHANGE_EVENT_ID = "change-page";
export const WEBSOCKET_UPDATE_ID = "websocket-update";

export const goToLobby = function () {
    window.history.pushState("/lobby", "LOBBY", "/lobby");
    const changeEvent = new Event(PAGE_CHANGE_EVENT_ID);
    window.dispatchEvent(changeEvent);
}

export const goToSignup = function () {
    window.history.pushState("/create-user", "CREATE USER", "/create-user");
    const changeEvent = new Event(PAGE_CHANGE_EVENT_ID);
    window.dispatchEvent(changeEvent);
}
