FROM rust:1.58-alpine as builder

ENV PATH="/app/bin:${PATH}"

RUN mkdir -pv /app
COPY ./Cargo.toml /app
COPY ./Cargo.lock /app
COPY ./config.json /app
COPY ./run-lupt.sh /app/bin/run-lupt.sh
COPY ./src /app/src
COPY ./static /app/static

RUN apk upgrade --update-cache --available
RUN apk add musl-dev openssl-dev

WORKDIR /app

RUN cargo build
RUN cp -v target/debug/lupt bin
RUN cargo clean

RUN chmod 755 bin/run-lupt.sh

CMD ["run-lupt.sh"]
