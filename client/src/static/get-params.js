export const getParams = function (match) {
    const values = match.result.slice(1);
    const keys = Array.from(match.route.name.matchAll(/:(\w+)/g)).map(result => result[1]);

    return Object.fromEntries(keys.map((key, i) => {
        return [key, values[i]];
    }));
};
