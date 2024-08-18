module.exports = {
    apps: [
        {
            name: "web-durak-client",
            script: "npm run start",
            watch_delay: 100000,
            ignore_watch : ["node_modules"],
            watch: false,
        },
        {
            name: "web-durak-server",
            script: "cargo run --release",
            watch_delay: 100000,
            watch: false,
        },
    ],
};
