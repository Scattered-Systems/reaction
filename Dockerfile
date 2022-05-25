FROM rust as builder

RUN update-ca-certificates

ENV PROJECT_SLUG=rustbox \
    USER=user \
    UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /$PROJECT_SLUG

COPY app/bin .
COPY config ./

RUN cargo build --release

FROM debian:buster-slim

ENV PROJECT_SLUG=rustbox \
    USER=user

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /$PROJECT_SLUG/target/release/$PROJECT_SLUG ./

USER $USER:$USER

EXPOSE 8080:8080
CMD ["rustbox\rustbox"]