FROM node:20-alpine

WORKDIR /usr/src/web-durak

EXPOSE ${CLIENT_PORT}

COPY package.json package-lock.json ./

RUN npm install

COPY . ./

CMD ["npm", "run", "start"]
