import * as path from "node:path";
import * as fs from "node:fs";
import { fileURLToPath } from 'url';
import { dirname } from 'path';
import {raiseNotFoundErr, raiseUnknownErr} from "./error-handling.js";
import {NotFoundPage} from "./static/pages/not-found-page/not-found-page.js";
import {ErrorPage} from "./static/pages/error-page/error-page.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const serveStatic = function (request, response) {
    try {
        const filePath = path.join(__dirname, "..", request.url);

        const extname = path.extname(filePath);
        let contentType = "";
        switch (extname) {
            case '.js':
                contentType = "text/javascript";
                break;
            case '.css':
                contentType = "text/css";
                break;
            case '.json':
                contentType = "application/json";
                break;
            case '.png':
                contentType = "image/png";
                break;
            case '.svg':
                contentType = "image/sgv";
                break;
            case '.ico':
                contentType = "image/ico";
                break;
            case '.jpg':
                contentType = "image/jpg";
                break;
            case '.wav':
                contentType = 'audio/wav';
                break;
        }

        fs.readFile(filePath, function (error, content) {
            if (error) {
                if (error.code === "ENOENT") {
                    response.writeHead(404);
                    return response.end(NotFoundPage.call({notFoundPage: request.url}));
                } else {
                    response.writeHead(500);
                    return response.end(ErrorPage.call({errorName: error.name, errorStack: error.stack}));
                }
            } else {
                response.writeHead(200, {'Content-Type': contentType});
                return response.end(content, 'utf-8');
            }
        });

    }
    catch (error) {
        console.log(error);
        throw error;
    }
}

export default serveStatic;
