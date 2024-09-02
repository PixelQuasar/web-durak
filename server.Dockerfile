FROM rust:1.67 as builder

WORKDIR /usr/src/web-durak

EXPOSE ${SERVER_PORT}

COPY ./ ./

#ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/web-durak/target/release/web-durak /
CMD ["./web-durak"]