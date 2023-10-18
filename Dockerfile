FROM rust:buster

WORKDIR /usr/src/rinhaRust
COPY . .

RUN cargo build
EXPOSE 8080

CMD ["target/debug/rinhaRust"]