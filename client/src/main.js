import {createApp} from "./app.js";
import * as dotenv from "dotenv";

dotenv.config();

const app = createApp();

app.listen(process.env.CLIENT_PORT);
