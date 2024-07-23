export const PAGE_CHANGE_EVENT_ID = "change-page";
export const WEBSOCKET_UPDATE_ID = "websocket-update";

export const PAGE_TITLE = "WEB DURAK";

export const navigate = function (path) {
    window.history.pushState("", PAGE_TITLE, path);
    const changeEvent = new Event(PAGE_CHANGE_EVENT_ID);
    window.dispatchEvent(changeEvent);
}
