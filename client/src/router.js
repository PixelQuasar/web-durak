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

    const getRoutes = function () {
        return routes
    }

    return { getRoutes };
}
