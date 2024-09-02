FROM ubuntu:latest
LABEL authors="quasarity"

ENTRYPOINT ["top", "-b"]