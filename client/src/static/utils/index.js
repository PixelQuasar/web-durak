export const PAGE_CHANGE_EVENT_ID = "change-page";
export const WEBSOCKET_UPDATE_ID = "websocket-update";
export const PAGE_TITLE = "WEB DURAK";
export const PAGE_RENDER_EVENT_ID = "render-page";

/**
 * Navigate to page by changing browser URL string.
 * @param {string} path
 */
export const navigate = function (path) {
    window.history.pushState("", PAGE_TITLE, path);
    const changeEvent = new Event(PAGE_CHANGE_EVENT_ID);
    window.dispatchEvent(changeEvent);
}

export const triggerRenderPageId = function () {
    const websocketEvent = new Event(PAGE_RENDER_EVENT_ID);
    window.dispatchEvent(websocketEvent);
}