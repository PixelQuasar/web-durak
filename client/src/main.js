import {createApp} from "./app.js";
import {createRouter} from "./router.js";
import {Homepage} from "./static/pages/homepage/homepage.js";
import {JoinLobby} from "./static/pages/join-lobby/join-lobby.js";
import {CreateLobby} from "./static/pages/create-lobby/create-lobby.js";

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

])

const app = createApp(router);

app.listen(3000);
