module.exports = {
    apps: [
        {
            name: "web-durak",
            script: ["cargo run --release", "npm run start"],
            "watch-ignore": ["/\\]./", "node_modules", "*.log", "public", "src"],
            watch: false,
        },
    ],
};
