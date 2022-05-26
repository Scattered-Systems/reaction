FROM rust as builder

RUN update-ca-certificates

ADD . /appseed
WORKDIR /appseed

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /appseed/target/release/bin /appseed/bin

EXPOSE 8080
CMD ["./appseed/bin"]