FROM rust:latest 

WORKDIR /usr/src/anonymity-network
COPY . .

RUN cargo build --release

CMD ["./target/release/anonymity-network"]