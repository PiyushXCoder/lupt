FROM rust:alpine3.18 AS build

RUN apk add --no-cache musl-dev pkgconfig openssl-dev 

WORKDIR /app
COPY . .

RUN cargo build --release 




FROM alpine:3.18

ARG RUNNER_GROUP_ID=local
ENV RUNNER_GROUP_ID=${RUNNER_GROUP_ID}

ARG RUNNER_USER_ID=local
ENV RUNNER_USER_ID=${RUNNER_USER_ID}

ARG APP=local
ENV APP=${APP}

RUN apk add --no-cache openssl 

RUN addgroup -g ${RUNNER_GROUP_ID} runner && adduser -G runner -u ${RUNNER_USER_ID} runner -D
USER runner

COPY --from=build --chown=runner:runner /app/target/release/${APP} /app/app
COPY --from=build --chown=runner:runner /app/static /app/static

WORKDIR /app

CMD ./app --bind-address 0.0.0.0 --port 8000 --config-file /app/config.json --static-path /app/static
