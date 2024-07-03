import './style.scss';
import {createRouter} from "./router.js";
import {Homepage} from "./pages/homepage/homepage.js";
import {JoinLobby} from "./pages/join-lobby/join-lobby.js";
import {CreateLobby} from "./pages/create-lobby/create-lobby.js";

const router = createRouter([
    {
        name: "/",
        page: Homepage
    },
    {
        name: "/join",
        page: JoinLobby
    },
    {
        name: "/create",
        page: CreateLobby
    }
]);

router.render();
