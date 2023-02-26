FROM rust:1-alpine3.16

ENV APP=lupt
ENV PORT=8081

RUN cargo search --limit 0 && \
    apk upgrade --update-cache --available && \
    apk add musl-dev && \
    apk add pkgconfig && \
    apk add openssl-dev && \
    rm -rf /var/cache/apk/* && \
    mkdir -pv /app/${APP}/etc

WORKDIR /app/${APP}
COPY . .

RUN cargo build --release && \
    cp target/release/lupt . && \
    cargo clean && \
    rm -rf /usr/local/rustup/ /usr/local/cargo/ && \
    apk del gcc

EXPOSE ${PORT}/tcp

CMD ./${APP} --bind-address 0.0.0.0 --port ${PORT} --config-file /app/${APP}/etc/config.json --static-path /app/${APP}/static/
