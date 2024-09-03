import {getParams} from "./get-params.js";
import {NotFoundPage} from "./pages/index.js";
import {PAGE_RENDER_EVENT_ID} from "./utils/index.js";

/**
 * transform path to regex
 * @param {string} path
 * @returns {RegExp}
 */
const pathToRegex = path => new RegExp("^" + path.replace(/\//g, "\\/").replace(/:\w+/g, "(.+)") + "$");

/**
 * creates client-side page router from pages tree.
 * @param {any} routesTree
 * @returns {{render: () => void}}
 */
export const createRouter = function (routesTree) {
    const routes = [];
    try {
        for (const route of routesTree) {
            if (route.name && route.page) {
                routes.push(route);
            }
        }
    }
    catch (error) {
        console.error("Error: unable to build router: ", error);
    }

    const render = function (globalProps = {}) {
        let match = routes
            .map(route => ({
                route: route,
                result: location.pathname.match(pathToRegex(route.name))
            }))
            .find(potentialMatch => potentialMatch.result !== null);

        if (!match) {
            match = {
                route: {
                    name: "404",
                    page: NotFoundPage.bind({notFoundPage: location.pathname})
                },
                result: [location.pathname]
            };
        }

        const root = document.querySelector("#root");
        root.innerHTML = match.route.page.bind({
            globalProps: globalProps,
            params: getParams(match)
        }).call();

        const changeEvent = new Event(PAGE_RENDER_EVENT_ID);
        document.dispatchEvent(changeEvent);
    }

    return { render };
}
