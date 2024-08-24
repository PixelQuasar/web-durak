# web-durak

### Multiplayer web implementation of traditional Russian card game via rust axum websocket server.

### Controls:
- Throw card: click on the card in your hand and then in the middle of thr table;
- Toss card: same as throw;
- Beat: click on the card in your hand and then on card which you want to beat.

### Rules:
https://en.wikipedia.org/wiki/Durak

### how to start:

#### Required services:
 - nodejs
 - rustc
 - redis
 - pm2

#### install node dependencies:
```
npm i
```

#### launch pm2 ecosystem:
```
pm2 start ecosystem.config.cjs
```



### .env example:
```dotenv
# server
SERVER_HOST="127.0.0.1"
SERVER_PORT="3500"
REDIS_CONNECTION_URL="redis://localhost"
PREFIX_LOBBY="1"
PREFIX_PLAYER="2"
API_PREFIX="/api"

# client
CLIENT_PORT=3000
SERVER_URL="/api"
WEBSOCKET_URL="/api"
```
