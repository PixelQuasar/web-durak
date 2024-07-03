import http from 'http';
import {NotFoundPage} from "./static/pages/not-found-page/not-found-page.js";
import {ErrorPage} from "./static/pages/error-page/error-page.js";

export const createApp = (appRouter) => {
    const router = appRouter;

    const requestHandler = function (req, res) {
        try {
            const page = router.getRoutes().find(item => item.name === req.url);
            if (!page) throw new Error("NOTFOUND");

            res.end(page.page());
            //res.end(page.page.apply({params: }));
        }
        catch (error) {
            let errorPage = null;
            if (error.message === "NOTFOUND") {
                errorPage = NotFoundPage.call({notFoundPage: req.url});
            }
            else {
                errorPage = ErrorPage.call({errorName: error.name, errorStack: error.stack});
            }
            res.end(errorPage);
        }
    }

    const listen = function (port, host = "localhost") {
        console.log(router);
        const server = http.createServer(requestHandler);
        server.listen(port, host, () => {
            console.log(`Server is running on http://${host}:${port}`);
        });
    }

    return { listen, requestHandler };
}
