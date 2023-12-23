FROM rust:alpine3.18 AS build
RUN cargo search --limit 0 && \
  apk add --no-cache musl-dev pkgconfig openssl-dev 
WORKDIR /app
COPY . .
RUN cargo build --release 




FROM alpine:3.18

ARG RUNNER_GROUP_ID=1000
ENV RUNNER_GROUP_ID=${RUNNER_GROUP_ID}

ARG RUNNER_USER_ID=1000
ENV RUNNER_USER_ID=${RUNNER_USER_ID}

ENV APP=lupt
ENV RUST_LOG="actix_web=info"

USER ${RUNNER_USER_ID}:${RUNNER_GROUP_ID}
WORKDIR /app
COPY --from=build --chown=${RUNNER_USER_ID}:${RUNNER_GROUP_ID} /app/target/release/${APP} app
COPY --from=build --chown=${RUNNER_USER_ID}:${RUNNER_GROUP_ID} /app/static static/
COPY --from=build --chown=${RUNNER_USER_ID}:${RUNNER_GROUP_ID} [ "/app/.config.json", "config.json" ]
CMD RUST_LOG="${RUST_LOG}" ./app --config-file config.json
