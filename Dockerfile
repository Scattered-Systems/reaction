FROM node:latest as contracts

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN npm install && npm run build

FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner

ENV RUST_LOG="info" \
    SERVER_PORT=9099

RUN apt-get update -y && apt-get upgrade -y 

COPY --chown=55 .config /config
VOLUME ["/config"]

COPY --from=builder /workspace/target/release/reaction /bin/reaction

FROM runner

EXPOSE 80
EXPOSE 6379
EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "reaction" ]
CMD [ "system", "--up" ]
