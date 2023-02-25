FROM rust:1-alpine3.16

ENV APP=lupt

RUN cargo search --limit 0

RUN apk upgrade --update-cache --available && \
    apk add musl-dev && \
    apk add pkgconfig && \
    apk add openssl-dev && \
    rm -rf /var/cache/apk/*

WORKDIR /app/${APP}
RUN mkdir /app/${APP}/etc

COPY . .

RUN cargo build --release
RUN cp target/release/lupt .
RUN cargo clean

RUN /bin/sh -c 'export FILE=~/.cargo/registry/cache/; if [ -e $FILE ] ; then rm -rf $FILE ; fi'
RUN /bin/sh -c 'export FILE=~/.cargo/registry/src/; if [ -e $FILE ] ; then rm -rf $FILE ; fi'
RUN rm -rf /usr/local/rustup/ /usr/local/cargo/
RUN apk del gcc

EXPOSE 8080/tcp

CMD ["${APP}", "--bind_address", "0.0.0.0", "--port", "8080", "--config-file", "/app/${APP}/etc/config.json", "--static_path", "/app/${APP}/static/"]
