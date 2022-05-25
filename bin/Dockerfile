FROM rust as builder

RUN update-ca-certificates

ADD . /bin
WORKDIR /bin

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /bin/target/release/reaction /bin/reaction
EXPOSE 8080:8080

CMD ["./bin/reaction"]