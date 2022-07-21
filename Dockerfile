FROM jo3mccain/rusty as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --verbose --color always

FROM photon as application

ENV PORT=9002

COPY --from=builder /app/target/release/reaction /reaction

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp
ENTRYPOINT ["./reaction"]