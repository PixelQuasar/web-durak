import http from "http";
import * as fs from "node:fs";
import {NotFoundPage} from "./static/pages/not-found-page/not-found-page.js";
import {ErrorPage} from "./static/pages/error-page/error-page.js";
import serveStatic from "./serve-static.js"
import path from "node:path";
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const INDEX_HTML_PATH = path.join(__dirname, "..", "public/index.html");

export const createApp = () => {
    const requestHandler = function (request, response) {
        try {
            if (request.url.includes("public") || request.url.includes("build") || request.url.includes("assets")) {
                return serveStatic(request, response);
            }

            const html = fs.readFileSync(INDEX_HTML_PATH, 'utf8')
            return response.end( html, 'utf-8');
        }
        catch (error) {
            console.log("ERROR!!");
            let errorPage = null;
            if (error.message === "NOTFOUND") {
                response.writeHead(404);
                errorPage = NotFoundPage.call({notFoundPage: request.url});
            }
            else {
                response.writeHead(500);
                errorPage = ErrorPage.call({errorName: error.name, errorStack: error.stack});
            }
            response.end(errorPage);
        }
    }

    const listen = function (port, host = "localhost") {
        const server = http.createServer(requestHandler);
        server.listen(port, host, () => {
            console.log(`Server is running on http://${host}:${port}`);
        });
    }

    return { listen, requestHandler };
}
