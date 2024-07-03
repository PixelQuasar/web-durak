import * as path from "node:path";
import * as fs from "node:fs";
import { fileURLToPath } from 'url';
import { dirname } from 'path';
import {raiseNotFoundErr, raiseUnknownErr} from "./error-handling.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const serveStatic = function (request, response) {
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

    fs.readFile(filePath, function(error, content) {
        if (error) {
            if(error.code === "ENOENT") {
                raiseNotFoundErr();
            }
            else {
                raiseUnknownErr();
            }
        }
        else {
            response.writeHead(200, { 'Content-Type': contentType });
            return response.end(content, 'utf-8');
        }
    });
}

export default serveStatic;
