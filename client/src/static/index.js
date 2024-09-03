import './style.scss';
import {createRouter} from "./router.js";
import {CreateLobby, CreateUser, Homepage, JoinLobby, LobbyPage, GamePage, VictoryPage} from "./pages/index.js";
import {PAGE_CHANGE_EVENT_ID, WEBSOCKET_UPDATE_ID} from "./utils/index.js";

const router = createRouter([
    {
        name: "/",
        page: Homepage
    },
    {
        name: "/join-lobby",
        page: JoinLobby
    },
    {
        name: "/join-lobby/:lobbyid",
        page: JoinLobby
    },
    {
        name: "/create-lobby",
        page: CreateLobby
    },
    {
        name: "/create-user",
        page: CreateUser
    },
    {
        name: "/lobby",
        page: LobbyPage
    },
    {
        name: "/game",
        page: GamePage
    },
    {
        name: "/scores",
        page: VictoryPage
    }
]);

router.render();

window.addEventListener(PAGE_CHANGE_EVENT_ID, () => {
    router.render();
});

window.addEventListener(WEBSOCKET_UPDATE_ID, () => {
   router.render();
});

window.addEventListener("popstate", () => {
    router.render();
})