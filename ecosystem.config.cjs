module.exports = {
    apps: [
        {
            name: "web-durak-client",
            script: "npm run start",
            watch: true,
        },
        {
            name: "web-durak-server",
            script: "cargo run --release",
            watch: true,
        },
    ],
};
