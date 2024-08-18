module.exports = {
    apps: [
        {
            name: "web-durak-client",
            script: "npm run start",
            watch: false,
        },
        {
            name: "web-durak-server",
            script: "cargo run --release",
            watch: false,
        },
    ],
};
