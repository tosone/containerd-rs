FROM rustlang/rust:nightly-buster

WORKDIR /app
COPY . .

RUN cargo build --verbose
